use rust_orm_gen::discovery::discover;
#[tokio::test]
#[ignore = "Requires the disposable Docker PostgreSQL fixture via TEST_DATABASE_URL"]
async fn postgres_catalog_preserves_relationship_evidence() {
    let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL required");
    let (mut client, connection) = tokio_postgres::connect(&url, tokio_postgres::NoTls)
        .await
        .unwrap();
    let task = tokio::spawn(async move { connection.await.unwrap() });
    client.batch_execute(r#"
 DROP SCHEMA IF EXISTS discovery_a CASCADE; DROP SCHEMA IF EXISTS discovery_b CASCADE;
 CREATE SCHEMA discovery_a; CREATE SCHEMA discovery_b;
 CREATE TABLE discovery_a.customers(tenant integer,id integer,PRIMARY KEY(tenant,id));
 CREATE TABLE discovery_b.customers(id integer PRIMARY KEY);
 CREATE TABLE discovery_a.orders(buyer integer,tenant integer,amount integer,active boolean,
 CONSTRAINT customer_fk FOREIGN KEY(buyer,tenant) REFERENCES discovery_a.customers(id,tenant) ON DELETE SET NULL ON UPDATE CASCADE DEFERRABLE INITIALLY DEFERRED);
 CREATE UNIQUE INDEX partial_key ON discovery_a.orders(buyer,tenant) WHERE active;
 CREATE TABLE discovery_a.profile(buyer integer,tenant integer,UNIQUE(buyer,tenant),FOREIGN KEY(buyer,tenant) REFERENCES discovery_a.customers(id,tenant));
 CREATE TABLE discovery_a.external(customer integer REFERENCES discovery_b.customers(id));
 CREATE TABLE discovery_a.unchecked(customer integer);
 INSERT INTO discovery_a.unchecked VALUES(999);
 ALTER TABLE discovery_a.unchecked ADD CONSTRAINT unchecked_fk FOREIGN KEY(customer) REFERENCES discovery_b.customers(id) NOT VALID;
 CREATE TABLE discovery_a."Quoted table"("Account Key" integer REFERENCES discovery_b.customers(id));
 DROP ROLE IF EXISTS analytics_catalog_reader;
 CREATE ROLE analytics_catalog_reader;
 GRANT USAGE ON SCHEMA discovery_a,discovery_b TO analytics_catalog_reader;
 GRANT SELECT ON ALL TABLES IN SCHEMA discovery_a,discovery_b TO analytics_catalog_reader;
 SET ROLE analytics_catalog_reader;
 "#).await.unwrap();
    let snapshot = discover(&mut client, &["discovery_a".into()])
        .await
        .unwrap();
    assert_eq!(snapshot.relationships.len(), 5);
    let orders = snapshot
        .relationships
        .iter()
        .find(|r| r.from_table == "orders")
        .unwrap();
    assert_eq!(orders.cardinality, "many-to-one");
    assert_eq!(
        orders
            .columns
            .iter()
            .map(|c| (c.from.as_str(), c.to.as_str()))
            .collect::<Vec<_>>(),
        vec![("buyer", "id"), ("tenant", "tenant")]
    );
    assert!(orders.columns.iter().all(|c| c.nullable));
    assert!(orders.deferrable && orders.initially_deferred && orders.validated && orders.enforced);
    assert_eq!(orders.on_delete, "set null");
    assert_eq!(orders.on_update, "cascade");
    assert_eq!(
        snapshot
            .relationships
            .iter()
            .find(|r| r.from_table == "profile")
            .unwrap()
            .cardinality,
        "one-to-one"
    );
    assert_eq!(
        snapshot
            .relationships
            .iter()
            .find(|r| r.from_table == "external")
            .unwrap()
            .to_schema,
        "discovery_b"
    );
    assert!(
        !snapshot
            .relationships
            .iter()
            .find(|r| r.from_table == "unchecked")
            .unwrap()
            .validated
    );
    assert!(snapshot
        .relationships
        .iter()
        .any(|r| r.columns[0].from == "Account Key"));
    assert!(!snapshot.keys.iter().any(|k| k.name == "partial_key"));
    let empty = discover(
        &mut client,
        &["discovery_a'); DROP SCHEMA discovery_a CASCADE; --".into()],
    )
    .await
    .unwrap();
    assert!(empty.relationships.is_empty());
    assert!(client
        .batch_execute("INSERT INTO discovery_b.customers VALUES(5)")
        .await
        .is_err());
    let snapshot2 = discover(&mut client, &["discovery_a".into()])
        .await
        .unwrap();
    assert_eq!(snapshot, snapshot2);
    client.batch_execute("RESET ROLE; REVOKE SELECT ON discovery_b.customers FROM analytics_catalog_reader; SET ROLE analytics_catalog_reader;").await.unwrap();
    let limited = discover(&mut client, &["discovery_a".into()])
        .await
        .unwrap();
    assert!(limited
        .relationships
        .iter()
        .all(|r| r.to_schema != "discovery_b"));
    drop(client);
    task.abort();
}
