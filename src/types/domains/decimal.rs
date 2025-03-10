use bytes::BytesMut;
use rust_decimal::Decimal;
use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::ops::Add;
use std::borrow::Cow;

use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DomainDecimal(pub Decimal);

impl Default for DomainDecimal {
    fn default() -> Self {
        Self(Decimal::default())
    }
}

 

// Implement Add trait for DomainDecimal
impl Add for DomainDecimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        DomainDecimal(self.0 + other.0)
    }
}

// Implement Add trait with reference for more efficient operations
impl<'a> Add<&'a DomainDecimal> for DomainDecimal {
    type Output = Self;
    
    fn add(self, other: &'a DomainDecimal) -> Self::Output {
        DomainDecimal(self.0 + other.0)
    }
}

// Implement Add trait with reference for more efficient operations
impl<'a> Add<DomainDecimal> for &'a DomainDecimal {
    type Output = DomainDecimal;
    
    fn add(self, other: DomainDecimal) -> Self::Output {
        DomainDecimal(self.0 + other.0)
    }
}

// Implement Add trait with references on both sides
impl<'a, 'b> Add<&'b DomainDecimal> for &'a DomainDecimal {
    type Output = DomainDecimal;
    
    fn add(self, other: &'b DomainDecimal) -> Self::Output {
        DomainDecimal(self.0 + other.0)
    }
}

use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::KnownFormat;
use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::RefOr;
use utoipa::openapi::Schema;
use utoipa::openapi::SchemaFormat;
use utoipa::PartialSchema;
use utoipa::ToSchema;

impl utoipa::ToSchema for DomainDecimal {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DomainDecimal")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}

impl utoipa::PartialSchema for DomainDecimal {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::Number))
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::Double)))
                .build(),
        ))
    }
}

impl ToSql for DomainDecimal {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        // Rust Decimal already has built-in support for PostgreSQL's NUMERIC
        <Decimal as ToSql>::to_sql(&self.0, ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        <Decimal as ToSql>::accepts(sql_type)
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for DomainDecimal {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        // Parse the NUMERIC value into a Decimal
        let decimal = <Decimal as FromSql>::from_sql(ty, raw)?;
        Ok(DomainDecimal(decimal))
    }

    fn accepts(sql_type: &Type) -> bool {
        <Decimal as FromSql>::accepts(sql_type)
    }
}

impl From<Decimal> for DomainDecimal {
    fn from(input: Decimal) -> Self {
        Self(input)
    }
}

impl Serialize for DomainDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize Decimal as a string for consistent representation
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for DomainDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DomainDecimalVisitor;

        impl<'de> Visitor<'de> for DomainDecimalVisitor {
            type Value = DomainDecimal;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a decimal number")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Decimal::from_str_exact(value)
                    .map(DomainDecimal)
                    .map_err(de::Error::custom)
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Decimal::try_from(value)
                    .map(DomainDecimal)
                    .map_err(de::Error::custom)
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DomainDecimal(Decimal::from(value)))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DomainDecimal(Decimal::from(value)))
            }
        }

        deserializer.deserialize_any(DomainDecimalVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use serde_test::{assert_de_tokens, Token};
    use std::str::FromStr;

    #[test]
    fn test_deserialize_from_string() {
        let expected = DomainDecimal(Decimal::from_str("123.456").unwrap());
        let tokens = &[Token::Str("123.456")];
        assert_de_tokens(&expected, tokens);
    }

    #[test]
    fn test_deserialize_from_integer() {
        let expected = DomainDecimal(Decimal::from(987654321i64));
        let tokens = &[Token::I64(987654321)];
        assert_de_tokens(&expected, tokens);
    }

    use bytes::BytesMut;
    use tokio_postgres::types::Type;

    fn simulate_numeric_roundtrip(
        value: &DomainDecimal,
    ) -> Result<DomainDecimal, Box<dyn Error + Sync + Send>> {
        let mut bytes = BytesMut::with_capacity(256);

        // Simulate to_sql
        value.to_sql(&Type::NUMERIC, &mut bytes)?;

        // Simulate from_sql
        DomainDecimal::from_sql(&Type::NUMERIC, &bytes)
    }

    #[test]
    fn test_numeric_roundtrip() {
        // Test with various decimal values
        let test_values = vec![
            DomainDecimal(Decimal::from_str("0").unwrap()),
            DomainDecimal(Decimal::from_str("123.456").unwrap()),
            DomainDecimal(Decimal::from_str("-987.654").unwrap()),
            DomainDecimal(Decimal::from_str("0.0000001").unwrap()),
            DomainDecimal(Decimal::MAX),
            DomainDecimal(Decimal::MIN),
        ];

        for value in test_values {
            let roundtrip = simulate_numeric_roundtrip(&value).unwrap();
            assert_eq!(value, roundtrip);
        }

        println!("All PostgreSQL NUMERIC roundtrip tests passed successfully");
    }
}
