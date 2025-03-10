use bytes::BytesMut;
use serde::Deserialize;
use serde::Serialize;

use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DomainBytes(pub Vec<u8>);

impl<'a> FromSql<'a> for DomainBytes {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(DomainBytes(raw.to_vec()))
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::BYTEA
    }
}

impl ToSql for DomainBytes {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        out.extend_from_slice(&self.0);
        Ok(IsNull::No)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::BYTEA
    }

    to_sql_checked!();
}
