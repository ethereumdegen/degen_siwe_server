use crate::types::domains::id::DomainId;
use crate::util::built_from_row::BuiltFromDbRow;
use tokio_postgres::Row;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SelectedRecord<T: BuiltFromDbRow> {
    pub id: DomainId,
    pub entry: T,
}

impl<T: BuiltFromDbRow> BuiltFromDbRow for SelectedRecord<T> {
    fn from_row(row: &Row) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self {
            id: row.get("id"),
            entry: T::from_row(row)?,
        })
    }
}
