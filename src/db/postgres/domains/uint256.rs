use ethers::types::U256;
use serde::de;
use serde::Serializer;
use serde::Deserializer;
use serde::de::Visitor;
use std::fmt;
use std::str::FromStr;
use rust_decimal::Decimal;
use serde::Serialize;
use serde::Deserialize;
use bytes::BytesMut;
 use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::SchemaFormat;
use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::KnownFormat;
use utoipa::PartialSchema;
use std::borrow::Cow;
use utoipa::openapi::RefOr;
use utoipa::openapi::Schema;
use utoipa::ToSchema;

use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug,Clone,Eq,PartialEq )]
pub struct DomainUint256(pub U256);



impl utoipa::ToSchema for DomainUint256 {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DomainUint256")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}

impl utoipa::PartialSchema for DomainUint256 {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type( SchemaType::Type(utoipa::openapi::Type::Integer ) )
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::Byte)))

                /*.items(Some(Box::new(RefOr::T(Schema::Object(
                    ObjectBuilder::new()
                        .schema_type( SchemaType::Type(utoipa::openapi::Type::Integer ) ) // Changed to String since U256 is represented as a string
                        .description(Some("U256 number as string"))
                        .build()
                )))))*/
                .build()
        ))
    }
}



impl<'a> FromSql<'a> for DomainUint256 {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        /* let s = <&str as FromSql>::from_sql(ty, raw)?;

        let u256_val = U256::from_str_radix(s.trim_start_matches("0x"), 16)?;

        Ok(DomainUint256(u256_val))
        */
        println!("from sql 1 ! ");
        let s = <Decimal as FromSql>::from_sql(ty, raw)?;

        // Convert raw byte slice from varchar to a Rust String
        // let s: String = <String as FromSql>::from_sql(ty, raw)?;

        println!("from sql 2 ! {:?}", s );

        let decimal_string = s.to_string()  ;

        // Now, you should parse this string into your DomainUint256 type
        // I'm not sure how your DomainUint256 type is structured, but here's a basic idea:
        let u256_value = U256::from_dec_str(&decimal_string)?; // Assuming a decimal string representation

        println!("from sql 3");

        Ok(DomainUint256(u256_value))
    }

    fn accepts(sql_type: &Type) -> bool {
      

          sql_type == &Type::NUMERIC

    }
}

impl ToSql for DomainUint256 {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        


        
         


        let uint_string = self.0.to_string();
        println!("uint string {}", uint_string.clone());
        <&Decimal as ToSql>::to_sql(& &Decimal::from_str(&uint_string) ?, ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::NUMERIC
    }

    to_sql_checked!();
}





impl Serialize for DomainUint256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize U256 as a hexadecimal string
        serializer.serialize_str(&format!("{}", self.0. to_string()  ))
    }
}
 


impl<'de> Deserialize<'de> for DomainUint256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DomainUint256Visitor;

        impl<'de> Visitor<'de> for DomainUint256Visitor {
            type Value = DomainUint256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or integer representing a U256 value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                U256::from_dec_str(value)
                    .map(DomainUint256)
                    .map_err(de::Error::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DomainUint256(U256::from(value)))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value < 0 {
                    Err(de::Error::custom("negative value cannot be converted to U256"))
                } else {
                    Ok(DomainUint256(U256::from(value as u64)))
                }
            }
        }

        deserializer.deserialize_any(DomainUint256Visitor)
    }
}


/* 
#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, Token};
    use ethers::types::U256;

    #[test]
    fn test_deserialize_from_string() {
 
        // The U256 value we expect after deserialization
        let expected = DomainUint256(U256::from(123456789u64));

        // The sequence of tokens representing the serialized form
        let tokens = &[Token::Str("123456789")];

        // Assert that deserializing these tokens produces the expected value
        assert_de_tokens(&expected, tokens);
    }

    #[test]
    fn test_deserialize_from_integer() {
        // The U256 value we expect after deserialization
        let expected = DomainUint256(U256::from(987654321u64));

        // The sequence of tokens representing the serialized form
        let tokens = &[Token::U64(987654321)];

        // Assert that deserializing these tokens produces the expected value
        assert_de_tokens(&expected, tokens);
    }
}
 */