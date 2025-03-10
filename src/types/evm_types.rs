 
use crate::types::domains::eth_address::DomainEthAddress;


use super::domains::uint256::DomainUint256;
use hex::{self, FromHex};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;

/*
#[derive(Serialize, Deserialize, Debug, Clone )]
pub struct RawTx {


    pub chain_id: i64 ,
    pub to_address: DomainEthAddress ,

    pub input_bytes: DomainBytes ,
    pub description: String,
    pub description_short: String,

    pub created_at: DateTime<Utc> ,

}*/

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct RawTx {
    pub chain_id: i64,
    pub to: DomainEthAddress,
    pub data: TransactionCalldata, //bytes
    pub value: Option<DomainUint256>, //pub description: String,
                                   //pub description_short: String,
                                   //pub created_at: DateTime<Utc>,
}

// Import ethers ABI component
use ethers::abi::Abi;
use log::warn;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RawTxError {
    #[error("Failed to load ABI: {0}")]
    AbiLoadError(String),

    #[error("Function 'createAndPayInvoice' not found in ABI")]
    FunctionNotFound,

    #[error("Failed to encode function call: {0}")]
    EncodingError(String),
}
 

#[derive(Debug, Clone, PartialEq, Eq, ToSchema)]
pub struct TransactionCalldata(pub Vec<u8>);

impl TransactionCalldata {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn from_hex(hex_str: &str) -> Result<Self, hex::FromHexError> {
        let cleaned = hex_str.trim_start_matches("0x");
        let bytes = Vec::from_hex(cleaned)?;
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn to_hex_string(&self) -> String {
        format!("0x{}", hex::encode(&self.0))
    }
}

impl Serialize for TransactionCalldata {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize to "0x" prefixed hex string
        serializer.serialize_str(&self.to_hex_string())
    }
}

struct TransactionCalldataVisitor;

impl<'de> Visitor<'de> for TransactionCalldataVisitor {
    type Value = TransactionCalldata;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hex string with 0x prefix representing transaction calldata")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        TransactionCalldata::from_hex(value).map_err(|e| E::custom(format!("Invalid hex: {}", e)))
    }
}

impl<'de> Deserialize<'de> for TransactionCalldata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TransactionCalldataVisitor)
    }
}

impl FromStr for TransactionCalldata {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TransactionCalldata::from_hex(s)
    }
}

impl AsRef<[u8]> for TransactionCalldata {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for TransactionCalldata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::domains::eth_address::DomainEthAddress;
    use crate::types::domains::uint256::DomainUint256;
    use serde_json::{json, Value};

    #[test]
    fn test_transaction_calldata_serialization() {
        let calldata = TransactionCalldata::new(vec![0xde, 0xad, 0xbe, 0xef]);
        let serialized = serde_json::to_string(&calldata).unwrap();
        assert_eq!(serialized, "\"0xdeadbeef\"");
    }

    #[test]
    fn test_transaction_calldata_deserialization() {
        let json_str = "\"0xdeadbeef\"";
        let calldata: TransactionCalldata = serde_json::from_str(json_str).unwrap();
        assert_eq!(calldata.0, vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_raw_tx_with_calldata() {
        let json_data = json!({
            "chain_id": 1,
            "to": "0x5AEDA56215b167893e80B4fE645BA6d5Bab767DE",
            "data": "0xdeadbeef",
            "value": "0x100"
        });

        let raw_tx: RawTx = serde_json::from_value(json_data).unwrap();

        // Validate the deserialized data
        assert_eq!(raw_tx.chain_id, 1);
        assert_eq!(
            raw_tx.to.0.to_string().to_lowercase(),
            "0x5aeda56215b167893e80b4fe645ba6d5bab767de"
        );
        assert_eq!(raw_tx.data.to_hex_string(), "0xdeadbeef");
        assert!(raw_tx.value.is_some());

        // Reserialize to verify
        let reserialized: Value = serde_json::to_value(raw_tx).unwrap();
        assert_eq!(reserialized["data"], "0xdeadbeef");
    }

    #[test]
    fn test_transaction_calldata_from_hex() {
        // Test valid hex with 0x prefix
        let calldata = TransactionCalldata::from_hex("0xabcdef").unwrap();
        assert_eq!(calldata.0, vec![0xab, 0xcd, 0xef]);

        // Test valid hex without 0x prefix
        let calldata = TransactionCalldata::from_hex("123456").unwrap();
        assert_eq!(calldata.0, vec![0x12, 0x34, 0x56]);

        // Test empty data
        //  let calldata = TransactionCalldata::from_hex("0x").unwrap();
        //    assert_eq!(calldata.0, vec![]);
    }
}

/*


#[cfg(test)]
mod tests {
    use crate::types::domains::bytes::DomainBytes;

    use super::*;
    use serde_json::json;
    use std::str::FromStr;

    #[test]
    fn test_json_deserialization() {
        let json_data = json!({
            "chain_id": 1,
            "to_address": "0x5AEDA56215b167893e80B4fE645BA6d5Bab767DE",
            "input_bytes": "0xdeadbeef", // Example raw bytes in hex
            "description": "Test Transaction",
            "description_short": "Tx Test",
            "created_at": "2024-02-20T15:30:00Z"
        });

        let raw_tx: RawTx = serde_json::from_value(json_data).expect("Failed to deserialize JSON");

        // Expected values
        let expected_chain_id = 1;
        let expected_to_address = DomainEthAddress("0x5AEDA56215b167893e80B4fE645BA6d5Bab767DE".parse().unwrap());
        let expected_raw_bytes = DomainBytes(hex::decode("deadbeef").unwrap()); // assuming Web3RawBytes wraps a Vec<u8>
        let expected_created_at = "2024-02-20T15:30:00Z".parse::<DateTime<Utc>>().unwrap();

        // Assertions
        assert_eq!(raw_tx.chain_id, expected_chain_id);
        assert_eq!(raw_tx.to_address, expected_to_address);
        assert_eq!(raw_tx.input_bytes, expected_raw_bytes); // Ensure this comparison is valid based on Web3RawBytes definition
        assert_eq!(raw_tx.description, "Test Transaction");
        assert_eq!(raw_tx.description_short, "Tx Test");
        assert_eq!(raw_tx.created_at, expected_created_at);
    }
}
*/
