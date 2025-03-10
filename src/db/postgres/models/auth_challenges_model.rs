use crate::types::domains::eth_address::DomainEthAddress;
use chrono::Utc;
use degen_sql::db::postgres::models::model::PostgresModelError;
use degen_sql::db::postgres::postgres_db::Database;
use ethers::types::Address;
use tokio_postgres::Row;

use serde::Serialize;

/// Represents an authentication challenge for signing in with an Ethereum address.
#[derive(Serialize, Clone, Debug)]
pub struct AuthChallenge {
    //  pub id: i32,
    pub public_address: DomainEthAddress,
    pub challenge: String,
    // pub created_at: DateTime<Utc>,
}

impl AuthChallenge {
    /// Constructs an `AuthChallenge` instance from a database row.
    pub fn from_row(row: &Row) -> Result<Self, PostgresModelError> {
        Ok(Self {
            //   id: row.try_get("id")?,
            public_address: row.try_get::<_, DomainEthAddress>("public_address")?,
            challenge: row.try_get("challenge")?,
            
        })
    }

    pub fn new(public_address: Address, service_name: impl ToString) -> Self {
        let unix_time = Utc::now().timestamp();

        Self {
            public_address: DomainEthAddress(public_address),
            challenge: Self::generate_challenge_text(public_address, service_name, unix_time)
                .into(),
        }
    }

    pub fn generate_challenge_text(
        public_address: Address,
        service_name:  impl ToString,
        unix_timestamp: i64,
    ) -> String {
        format!(
            "Signing in to {} as {:?} at {}",
            service_name.to_string(), public_address, unix_timestamp
        )
    }
}

/// Model handling `AuthChallenge` interactions with the database.
pub struct AuthChallengesModel {}

impl AuthChallengesModel {
    /// Creates a new authentication challenge or updates an existing one.
    pub async fn insert_one(
        new_challenge: AuthChallenge,
        psql_db: &Database,
    ) -> Result<i32, PostgresModelError> {
        let insert_result = psql_db
            .query_one(
                "INSERT INTO challenge_tokens (public_address, challenge) 
                 VALUES ($1, $2) 
                
                 RETURNING id;",
                &[&new_challenge.public_address, &new_challenge.challenge],
            )
            .await;

        match insert_result {
            Ok(row) => Ok(row.get("id")),
            Err(e) => {
                eprintln!("{}", e);
                Err(e.into())
            }
        }
    }

    /// Retrieves the latest active challenge for a given public address.
    pub async fn find_one(
        public_address: &Address,
        challenge: &String,
        psql_db: &Database,
    ) -> Result<AuthChallenge, PostgresModelError> {
        let result = psql_db
            .query_one(
                "SELECT *
                 FROM challenge_tokens 
                 WHERE public_address = $1 AND challenge = $2
                 ORDER BY created_at DESC 
                 LIMIT 1",
                &[&DomainEthAddress(*public_address), &challenge],
            )
            .await;

        match result {
            Ok(row) => AuthChallenge::from_row(&row),
            Err(e) => {
                eprintln!("{}", e);
                Err(e.into())
            }
        }
    }
}
