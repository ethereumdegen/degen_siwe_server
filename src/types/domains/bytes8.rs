use serde::de;
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
use rand::{thread_rng, Rng};
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;

use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DomainBytes8(pub [u8; 8]);

impl utoipa::ToSchema for DomainBytes8 {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DomainBytes8")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}

impl utoipa::PartialSchema for DomainBytes8 {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::String))
                .description(Some("8-byte value represented as a hex string"))
                .build(),
        ))
    }
}

// Custom serialization to convert to hex string
impl Serialize for DomainBytes8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as hex string with 0x prefix
        serializer.serialize_str(&self.to_hex())
    }
}

// Custom deserialization to parse from hex string
impl<'de> Deserialize<'de> for DomainBytes8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Define a visitor that will handle the deserialization
        struct DomainBytes8Visitor;

        impl<'de> Visitor<'de> for DomainBytes8Visitor {
            type Value = DomainBytes8;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a hex string representing 8 bytes")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                DomainBytes8::from_hex(value).map_err(de::Error::custom)
            }
        }

        // Use the visitor to deserialize the value
        deserializer.deserialize_str(DomainBytes8Visitor)
    }
}

impl<'a> FromSql<'a> for DomainBytes8 {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        // First get the TEXT value from the database
        let s = <&str as FromSql>::from_sql(ty, raw)?;

        // Use the existing from_hex method for consistency
        DomainBytes8::from_hex(s)
    }

    fn accepts(sql_type: &Type) -> bool {
        // Accept TEXT or VARCHAR types
        sql_type == &Type::TEXT || sql_type == &Type::VARCHAR
    }
}

impl ToSql for DomainBytes8 {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        // Use the existing to_hex method for consistency
        let hex_string = self.to_hex();

        // Store as TEXT
        <&str as ToSql>::to_sql(&hex_string.as_str(), ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        // Accept TEXT or VARCHAR types
        sql_type == &Type::TEXT || sql_type == &Type::VARCHAR
    }

    to_sql_checked!();
}

// Convenience methods for DomainBytes8
impl DomainBytes8 {
    pub fn from_hex(hex: &str) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let clean_hex = hex.trim_start_matches("0x");

        let bytes =
            hex::decode(clean_hex).map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send>)?;

        if bytes.len() != 8 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Expected 8 bytes, got {}", bytes.len()),
            )));
        }

        let mut array = [0u8; 8];
        array.copy_from_slice(&bytes);

        Ok(DomainBytes8(array))
    }

    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.0))
    }

    /// Generate a random DomainBytes8 value
    pub fn random() -> Self {
        let mut random_bytes = [0u8; 8];
        thread_rng().fill(&mut random_bytes);
        DomainBytes8(random_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        let hex_str = "0x0001020304050607";
        let domain_bytes = DomainBytes8::from_hex(hex_str).unwrap();

        let expected: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(domain_bytes.0, expected);
    }

    #[test]
    fn test_to_hex() {
        let bytes: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        let domain_bytes = DomainBytes8(bytes);
        let hex_str = domain_bytes.to_hex();

        assert_eq!(hex_str, "0x0001020304050607");
    }
}
