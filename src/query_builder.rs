use std::marker::PhantomData;
use std::fmt;
use tokio_postgres::types::ToSql;

pub trait Model {
    fn table_name() -> &'static str;
    fn columns() -> &'static [&'static str];
}

pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

impl fmt::Display for JoinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JoinType::Inner => write!(f, "INNER JOIN"),
            JoinType::Left => write!(f, "LEFT JOIN"),
            JoinType::Right => write!(f, "RIGHT JOIN"),
            JoinType::Full => write!(f, "FULL JOIN"),
        }
    }
}

pub enum AggregateFunction {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

impl fmt::Display for AggregateFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AggregateFunction::Count => write!(f, "COUNT"),
            AggregateFunction::Sum => write!(f, "SUM"),
            AggregateFunction::Avg => write!(f, "AVG"),
            AggregateFunction::Min => write!(f, "MIN"),
            AggregateFunction::Max => write!(f, "MAX"),
        }
    }
}

pub struct Select<T: Model> {
    fields: Vec<String>,
    table: String,
    joins: Vec<(JoinType, String, String)>,
    conditions: Vec<String>,
    order_by: Vec<String>,
    group_by: Vec<String>,
    having: Vec<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    params: Vec<Box<dyn ToSql + Sync>>,
    _phantom: PhantomData<T>,
}

impl<T: Model> Select<T> {
    pub fn new() -> Self {
        Select {
            fields: vec!["*".to_string()],
            table: T::table_name().to_string(),
            joins: Vec::new(),
            conditions: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            limit: None,
            offset: None,
            params: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn select(mut self, fields: &[&str]) -> Self {
        for field in fields {
            if !T::columns().contains(field) {
                panic!("Field '{}' does not exist in table '{}'", field, T::table_name());
            }
        }
        self.fields = fields.iter().map(|&s| s.to_string()).collect();
        self
    }

    pub fn join(mut self, join_type: JoinType, table: &str, condition: &str) -> Self {
        self.joins.push((join_type, table.to_string(), condition.to_string()));
        self
    }

    pub fn where_clause(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }

    pub fn order_by(mut self, field: &str, asc: bool) -> Self {
        if !T::columns().contains(&field) {
            panic!("Field '{}' does not exist in table '{}'", field, T::table_name());
        }
        let direction = if asc { "ASC" } else { "DESC" };
        self.order_by.push(format!("{} {}", field, direction));
        self
    }

    pub fn group_by(mut self, fields: &[&str]) -> Self {
        for field in fields {
            if !T::columns().contains(field) {
                panic!("Field '{}' does not exist in table '{}'", field, T::table_name());
            }
        }
        self.group_by.extend(fields.iter().map(|&s| s.to_string()));
        self
    }

    pub fn having(mut self, condition: &str) -> Self {
        self.having.push(condition.to_string());
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn aggregate(mut self, function: AggregateFunction, field: &str, alias: Option<&str>) -> Self {
        if !T::columns().contains(&field) {
            panic!("Field '{}' does not exist in table '{}'", field, T::table_name());
        }
        let agg_field = match alias {
            Some(a) => format!("{}({}) AS {}", function, field, a),
            None => format!("{}({})", function, field),
        };
        self.fields.push(agg_field);
        self
    }

    pub fn bind_param<P: ToSql + Sync + 'static>(mut self, param: P) -> Self {
        self.params.push(Box::new(param));
        self
    }

    pub fn build(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let mut query = format!("SELECT {} FROM {}", self.fields.join(", "), self.table);

        for (join_type, table, condition) in &self.joins {
            query += &format!(" {} {} ON {}", join_type, table, condition);
        }

        if !self.conditions.is_empty() {
            query += &format!(" WHERE {}", self.conditions.join(" AND "));
        }

        if !self.group_by.is_empty() {
            query += &format!(" GROUP BY {}", self.group_by.join(", "));
        }

        if !self.having.is_empty() {
            query += &format!(" HAVING {}", self.having.join(" AND "));
        }

        if !self.order_by.is_empty() {
            query += &format!(" ORDER BY {}", self.order_by.join(", "));
        }

        if let Some(limit) = self.limit {
            query += &format!(" LIMIT {}", limit);
        }

        if let Some(offset) = self.offset {
            query += &format!(" OFFSET {}", offset);
        }

        let params: Vec<&(dyn ToSql + Sync)> = self.params.iter().map(|p| p.as_ref()).collect();
        (query, params)
    }
}

pub struct QueryBuilder;

impl QueryBuilder {
    pub fn select<T: Model>() -> Select<T> {
        Select::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestModel;

    impl Model for TestModel {
        fn table_name() -> &'static str {
            "users"
        }

        fn columns() -> &'static [&'static str] {
            &["id", "name", "email", "age"]
        }
    }

    #[test]
    fn test_select_query_builder() {
        let query_builder = QueryBuilder::select::<TestModel>()
            .select(&["name", "email"])
            .join(JoinType::Inner, "orders", "users.id = orders.user_id")
            .where_clause("age > $1")
            .group_by(&["name", "email"])
            .having("COUNT(orders.id) > $2")
            .order_by("name", true)
            .limit(10)
            .offset(5)
            .aggregate(AggregateFunction::Count, "id", Some("user_count"))
            .bind_param(18)
            .bind_param(5);

        let (query, params) = query_builder.build();

        assert_eq!(
            query,
            "SELECT name, email, COUNT(id) AS user_count FROM users INNER JOIN orders ON users.id = orders.user_id WHERE age > $1 GROUP BY name, email HAVING COUNT(orders.id) > $2 ORDER BY name ASC LIMIT 10 OFFSET 5"
        );
        assert_eq!(params.len(), 2);
    }   
}
