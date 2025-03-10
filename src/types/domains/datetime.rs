use std::borrow::Cow;
use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::KnownFormat;
use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::RefOr;
use utoipa::openapi::Schema;
use utoipa::openapi::SchemaFormat;
use utoipa::PartialSchema;
use utoipa::ToSchema;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DomainDatetime(pub DateTime<Utc>);

impl utoipa::ToSchema for DomainDatetime {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DomainDatetime")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}

impl utoipa::PartialSchema for DomainDatetime {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::String))
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::DateTime)))
                .description(Some("UTC date and time value"))
                .build(),
        ))
    }
}

impl ToSql for DomainDatetime {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut tokio_postgres::types::private::BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        // Use the inner DateTime's implementation
        self.0.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        // Accept the same types as DateTime<Utc>
        <DateTime<Utc> as ToSql>::accepts(ty)
    }

    tokio_postgres::types::to_sql_checked!();
}

impl<'a> FromSql<'a> for DomainDatetime {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        // Get DateTime<Utc> from the raw bytes, then wrap it
        let dt = <DateTime<Utc> as FromSql>::from_sql(ty, raw)?;
        Ok(DomainDatetime(dt))
    }

    fn accepts(ty: &Type) -> bool {
        // Accept the same types as DateTime<Utc>
        <DateTime<Utc> as FromSql>::accepts(ty)
    }
}

// Optional: Implement Display for better debugging and error messages
impl std::fmt::Display for DomainDatetime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

// Optional: Implement Deref for easier access to inner DateTime methods
impl std::ops::Deref for DomainDatetime {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Optional: Conversions from/to DateTime<Utc>
impl From<DateTime<Utc>> for DomainDatetime {
    fn from(dt: DateTime<Utc>) -> Self {
        DomainDatetime(dt)
    }
}

impl From<DomainDatetime> for DateTime<Utc> {
    fn from(domain_dt: DomainDatetime) -> Self {
        domain_dt.0
    }
}
