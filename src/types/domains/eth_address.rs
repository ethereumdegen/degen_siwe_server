use bytes::BytesMut;
use ethabi::Address;
use ethers::types::H160;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::{error::Error, str::FromStr};
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use utoipa::openapi::schema::SchemaType;
use utoipa::openapi::KnownFormat;
use utoipa::openapi::ObjectBuilder;
use utoipa::openapi::RefOr;
use utoipa::openapi::Schema;
use utoipa::openapi::SchemaFormat;
use utoipa::PartialSchema;
use utoipa::ToSchema;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DomainEthAddress(pub Address);

// ???
impl utoipa::PartialSchema for DomainEthAddress {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::String))
                .description(Some("Ethereum address in hex format"))
                .build(),
        ))
    }
}

impl utoipa::ToSchema for DomainEthAddress {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DomainEthAddress")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.push((Self::name().to_string(), Self::schema()));
    }
}

impl<'a> FromSql<'a> for DomainEthAddress {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = <&str as FromSql>::from_sql(ty, raw)?;

        let address = Address::from_str(s)?;
        //   let address_string = to_checksum(s, None)

        Ok(DomainEthAddress(address))
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::VARCHAR || sql_type == &Type::TEXT
    }
}

impl ToSql for DomainEthAddress {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        let address_string = self.to_string_full();

        println!("inserting {}", address_string);

        <&str as ToSql>::to_sql(&address_string.as_str(), ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::VARCHAR || sql_type == &Type::TEXT
    }

    to_sql_checked!();
}

impl DomainEthAddress {
    pub fn to_string_full(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl From<Address> for DomainEthAddress {
    fn from(input: H160) -> Self {
        Self(input)
    }
}
