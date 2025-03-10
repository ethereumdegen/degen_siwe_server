use bytes::BytesMut;
use ethers::types::H256;
use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::str::FromStr;

use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DomainH256(pub H256);

use std::borrow::Cow;
use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::KnownFormat;
use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::RefOr;
use utoipa::openapi::Schema;
use utoipa::openapi::SchemaFormat;
use utoipa::PartialSchema;
use utoipa::ToSchema;

impl utoipa::ToSchema for DomainH256 {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DomainH256")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}

impl utoipa::PartialSchema for DomainH256 {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::String))
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::Byte)))
                .description(Some("Ethereum H256 hash"))
                .build(),
        ))
    }
}

impl<'a> FromSql<'a> for DomainH256 {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = <&str as FromSql>::from_sql(ty, raw)?;

        // Handle both with and without '0x' prefix
        let hash_str = if s.starts_with("0x") { &s[2..] } else { s };

        let h256_val =
            H256::from_str(hash_str).map_err(|e| Box::new(e) as Box<dyn Error + Sync + Send>)?;

        Ok(DomainH256(h256_val))
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::VARCHAR || sql_type == &Type::TEXT
    }
}

impl ToSql for DomainH256 {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        // Convert H256 to hex string without '0x' prefix for storage
        let hex_string = format!("{:x}", self.0);
        <&str as ToSql>::to_sql(&hex_string.as_str(), ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::VARCHAR || sql_type == &Type::TEXT
    }

    to_sql_checked!();
}

impl Serialize for DomainH256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize H256 as a hexadecimal string with '0x' prefix
        serializer.serialize_str(&format!("0x{:x}", self.0))
    }
}

impl<'de> Deserialize<'de> for DomainH256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DomainH256Visitor;

        impl<'de> Visitor<'de> for DomainH256Visitor {
            type Value = DomainH256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an Ethereum H256 hash")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                // Handle both with and without '0x' prefix
                let hash_str = if value.starts_with("0x") {
                    &value[2..]
                } else {
                    value
                };

                H256::from_str(hash_str)
                    .map(DomainH256)
                    .map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(DomainH256Visitor)
    }
}

/*

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, Token};
    use ethers::types::H256;

    #[test]
    fn test_deserialize_from_string() {
        // The H256 value we expect after deserialization
        let hash_str = "000000000000000000000000000000000000000000000000000000000000abcd";
        let expected = DomainH256(H256::from_str(hash_str).unwrap());

        // Test with 0x prefix
        let tokens_with_prefix = &[Token::Str(&format!("0x{}", hash_str))];
        assert_de_tokens(&expected, tokens_with_prefix);

        // Test without 0x prefix
        let tokens_without_prefix = &[Token::Str(hash_str)];
        assert_de_tokens(&expected, tokens_without_prefix);
    }

    #[test]
    fn test_serialization() {
        let hash_str = "000000000000000000000000000000000000000000000000000000000000abcd";
        let hash = H256::from_str(hash_str).unwrap();
        let domain_hash = DomainH256(hash);

        let serialized = serde_json::to_string(&domain_hash).unwrap();
        assert_eq!(serialized, format!("\"0x{}\"", hash_str));
    }
} */
