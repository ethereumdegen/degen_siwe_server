use std::borrow::Cow;
use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::KnownFormat;
use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::RefOr;
use utoipa::openapi::Schema;
use utoipa::openapi::SchemaFormat;
use utoipa::PartialSchema;
use utoipa::ToSchema;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

impl Display for DomainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DomainId(pub i32);

impl utoipa::ToSchema for DomainId {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("id(i32)")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}

impl utoipa::PartialSchema for DomainId {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::Integer))
                .description(Some("Database record identifier"))
                .build(),
        ))
    }
}

impl From<i32> for DomainId {
    fn from(value: i32) -> Self {
        DomainId(value)
    }
}

impl From<DomainId> for i32 {
    fn from(value: DomainId) -> Self {
        value.0
    }
}

// Implementation for ToSql trait
impl ToSql for DomainId {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut bytes::BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        <i32 as ToSql>::to_sql(&self.0, ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        <i32 as ToSql>::accepts(ty)
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut bytes::BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        <i32 as ToSql>::to_sql_checked(&self.0, ty, out)
    }
}

// Implementation for FromSql trait
impl<'a> FromSql<'a> for DomainId {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let val = <i32 as FromSql>::from_sql(ty, raw)?;
        Ok(DomainId(val))
    }

    fn accepts(ty: &Type) -> bool {
        <i32 as FromSql>::accepts(ty)
    }
}
