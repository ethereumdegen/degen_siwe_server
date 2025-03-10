use std::borrow::Cow;
use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::KnownFormat;
use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::RefOr;
use utoipa::openapi::Schema;
use utoipa::openapi::SchemaFormat;
use utoipa::PartialSchema;
use utoipa::ToSchema;

use bytes::BytesMut;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Value};
use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainJson(pub Value);

impl utoipa::ToSchema for DomainJson {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DomainJson")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}

impl utoipa::PartialSchema for DomainJson {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::Object))
                .description(Some("JSON data"))
                .build(),
        ))
    }
}

impl<'a> FromSql<'a> for DomainJson {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        // For JSONB format, the first byte is the version number (currently 1)
        let json_str = if ty.name() == "jsonb" && !raw.is_empty() {
            std::str::from_utf8(&raw[1..])?
        } else {
            std::str::from_utf8(raw)?
        };

        let json_value = serde_json::from_str(json_str)?;
        Ok(DomainJson(json_value))
    }

    fn accepts(sql_type: &Type) -> bool {
        matches!(sql_type.name(), "json" | "jsonb")
    }
}

impl ToSql for DomainJson {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        // Convert Value to JSON string
        let json_string = to_string(&self.0)?;

        if ty.name() == "jsonb" {
            // JSONB format includes a version byte (1) at the beginning
            out.extend_from_slice(&[1]);
            out.extend_from_slice(json_string.as_bytes());
        } else {
            // JSON is just the string
            out.extend_from_slice(json_string.as_bytes());
        }

        Ok(IsNull::No)
    }

    fn accepts(sql_type: &Type) -> bool {
        matches!(sql_type.name(), "json" | "jsonb")
    }

    to_sql_checked!();
}

// Example implementation for convenience methods
impl DomainJson {
    pub fn new(value: Value) -> Self {
        DomainJson(value)
    }

    pub fn get(&self) -> &Value {
        &self.0
    }

    pub fn into_inner(self) -> Value {
        self.0
    }

    pub fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        let value = serde_json::from_str(s)?;
        Ok(DomainJson(value))
    }
}
