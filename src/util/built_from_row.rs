use tokio_postgres::Row;

pub trait BuiltFromDbRow {
    fn from_row(row: &Row) -> Option<Self>
    where
        Self: Sized;
}
