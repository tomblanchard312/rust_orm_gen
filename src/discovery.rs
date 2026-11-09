//! Read-only PostgreSQL catalog discovery. The caller owns connection/TLS policy.
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, IsolationLevel};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Key {
    pub schema: String,
    pub table: String,
    pub name: String,
    pub columns: Vec<String>,
    pub primary: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ColumnPair {
    pub from: String,
    pub to: String,
    pub nullable: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForeignKey {
    pub name: String,
    pub from_schema: String,
    pub from_table: String,
    pub to_schema: String,
    pub to_table: String,
    pub columns: Vec<ColumnPair>,
    pub cardinality: String,
    pub validated: bool,
    pub enforced: bool,
    pub deferrable: bool,
    pub initially_deferred: bool,
    pub on_update: String,
    pub on_delete: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchemaSnapshot {
    pub keys: Vec<Key>,
    pub relationships: Vec<ForeignKey>,
}
const KEYS: &str = r#"
SELECT n.nspname::text,c.relname::text,ic.relname::text,i.indisprimary,
 ARRAY(SELECT a.attname::text FROM unnest(i.indkey::smallint[]) WITH ORDINALITY k(attnum,ord)
 JOIN pg_attribute a ON a.attrelid=c.oid AND a.attnum=k.attnum
 WHERE k.ord<=i.indnkeyatts ORDER BY k.ord)
FROM pg_index i JOIN pg_class c ON c.oid=i.indrelid JOIN pg_namespace n ON n.oid=c.relnamespace
JOIN pg_class ic ON ic.oid=i.indexrelid
WHERE i.indisunique AND i.indisvalid AND i.indpred IS NULL AND i.indexprs IS NULL
 AND n.nspname !~ '^pg_' AND n.nspname<>'information_schema'
 AND (cardinality($1::text[])=0 OR n.nspname=ANY($1)) AND has_table_privilege(c.oid,'SELECT')
ORDER BY n.nspname,c.relname,ic.relname LIMIT 10001
"#;
const FKS: &str = r#"
SELECT c.conname::text,n.nspname::text,t.relname::text,rn.nspname::text,rt.relname::text,
 ARRAY(SELECT a.attname::text FROM unnest(c.conkey) WITH ORDINALITY k(attnum,ord) JOIN pg_attribute a ON a.attrelid=t.oid AND a.attnum=k.attnum ORDER BY k.ord),
 ARRAY(SELECT a.attname::text FROM unnest(c.confkey) WITH ORDINALITY k(attnum,ord) JOIN pg_attribute a ON a.attrelid=rt.oid AND a.attnum=k.attnum ORDER BY k.ord),
 ARRAY(SELECT NOT a.attnotnull FROM unnest(c.conkey) WITH ORDINALITY k(attnum,ord) JOIN pg_attribute a ON a.attrelid=t.oid AND a.attnum=k.attnum ORDER BY k.ord),
 c.convalidated,COALESCE((to_jsonb(c)->>'conenforced')::boolean,true),c.condeferrable,c.condeferred,c.confupdtype::text,c.confdeltype::text
FROM pg_constraint c JOIN pg_class t ON t.oid=c.conrelid JOIN pg_namespace n ON n.oid=t.relnamespace
JOIN pg_class rt ON rt.oid=c.confrelid JOIN pg_namespace rn ON rn.oid=rt.relnamespace
WHERE c.contype='f' AND n.nspname !~ '^pg_' AND n.nspname<>'information_schema'
 AND (cardinality($1::text[])=0 OR n.nspname=ANY($1))
 AND has_table_privilege(t.oid,'SELECT') AND has_table_privilege(rt.oid,'SELECT')
ORDER BY n.nspname,t.relname,c.conname LIMIT 10001
"#;
fn action(code: &str) -> String {
    match code {
        "a" => "no action",
        "r" => "restrict",
        "c" => "cascade",
        "n" => "set null",
        "d" => "set default",
        _ => "unknown",
    }
    .into()
}
/// Only SELECTs in a bounded, repeatable-read, read-only transaction. Empty schemas selects user schemas.
pub async fn discover(
    client: &mut Client,
    schemas: &[String],
) -> Result<SchemaSnapshot, crate::error::OrmError> {
    if schemas.len() > 100
        || schemas
            .iter()
            .any(|s| s.is_empty() || s.len() > 128 || s.contains('\0'))
    {
        return Err(crate::error::OrmError::ParseError(
            "Invalid schema selection".into(),
        ));
    }
    let transaction = client
        .build_transaction()
        .read_only(true)
        .isolation_level(IsolationLevel::RepeatableRead)
        .start()
        .await?;
    transaction
        .batch_execute("SET LOCAL statement_timeout = '5s'")
        .await?;
    let rows = transaction.query(KEYS, &[&schemas]).await?;
    if rows.len() > 10000 {
        return Err(crate::error::OrmError::ParseError(
            "Catalog key limit exceeded".into(),
        ));
    }
    let keys: Vec<Key> = rows
        .iter()
        .map(|r| Key {
            schema: r.get(0),
            table: r.get(1),
            name: r.get(2),
            primary: r.get(3),
            columns: r.get(4),
        })
        .collect();
    let rows = transaction.query(FKS, &[&schemas]).await?;
    if rows.len() > 10000 {
        return Err(crate::error::OrmError::ParseError(
            "Catalog relationship limit exceeded".into(),
        ));
    }
    let relationships = rows
        .iter()
        .map(|r| {
            let from_schema: String = r.get(1);
            let from_table: String = r.get(2);
            let local: Vec<String> = r.get(5);
            let remote: Vec<String> = r.get(6);
            let nullable: Vec<bool> = r.get(7);
            let unique = keys.iter().any(|k| {
                k.schema == from_schema
                    && k.table == from_table
                    && !k.columns.is_empty()
                    && k.columns.iter().all(|c| local.contains(c))
            });
            ForeignKey {
                name: r.get(0),
                from_schema,
                from_table,
                to_schema: r.get(3),
                to_table: r.get(4),
                columns: local
                    .into_iter()
                    .zip(remote)
                    .zip(nullable)
                    .map(|((from, to), nullable)| ColumnPair { from, to, nullable })
                    .collect(),
                cardinality: if unique { "one-to-one" } else { "many-to-one" }.into(),
                validated: r.get(8),
                enforced: r.get(9),
                deferrable: r.get(10),
                initially_deferred: r.get(11),
                on_update: action(r.get::<_, String>(12).as_str()),
                on_delete: action(r.get::<_, String>(13).as_str()),
            }
        })
        .collect();
    transaction.commit().await?;
    Ok(SchemaSnapshot {
        keys,
        relationships,
    })
}
