pub trait Model {
    fn table_name() -> &'static str;
    fn columns() -> &'static [&'static str];
}