use crate::app_state::AppState;
use actix_web::web::Data;
use serde::Deserialize;

use crate::types::domains::eth_address::DomainEthAddress;
use chrono::{DateTime, Utc};
use degen_sql::db::postgres::models::model::PostgresModelError;
use degen_sql::db::postgres::postgres_db::Database;
use ethers::types::Address;
use rand::Rng;
use serde::Serialize;
use tokio_postgres::Row;

/// Represents an authentication session for a signed-in Ethereum address.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthSession {
    pub public_address: DomainEthAddress,
    pub session_token: String,
    pub expires_at: DateTime<Utc>,
}

impl AuthSession {
    /// Constructs an `AuthSession` instance from a database row.
    pub fn from_row(row: &Row) -> Result<Self, PostgresModelError> {
        Ok(Self {
            public_address: row.try_get::<_, DomainEthAddress>("public_address")?,
            session_token: row.try_get("session_token")?,
            expires_at: row.try_get("expires_at")?,
        })
    }

    /// Creates a new session for a given public address.
    pub fn new(public_address: Address, expires_in_days: i64) -> Self {
        let session_token = Self::generate_session_token();
        let expires_at = Utc::now() + chrono::Duration::days(expires_in_days);

        Self {
            public_address: DomainEthAddress(public_address),
            session_token,
            expires_at,
        }
    }

    /// Generates a new random session token.
    fn generate_session_token() -> String {
        let mut rng = rand::thread_rng();
        (0..32)
            .map(|_| rng.gen_range(0..16))
            .map(|v| format!("{:x}", v))
            .collect()
    }
}

/// Model handling `AuthSession` interactions with the database.
pub struct AuthSessionsModel {}

impl AuthSessionsModel {
    /// Inserts a new session into the database.
    pub async fn insert_one(
        new_session: AuthSession,
        psql_db: &Database,
    ) -> Result<i32, PostgresModelError> {
        let insert_result = psql_db
            .query_one(
                "INSERT INTO user_sessions (public_address, session_token, expires_at) 
                 VALUES ($1, $2, $3) 
                 RETURNING id;",
                &[
                    &new_session.public_address,
                    &new_session.session_token,
                    &new_session.expires_at,
                ],
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
 
    pub async fn find_one(
        session_token: String,
        psql_db: &Database,
    ) -> Result<AuthSession, PostgresModelError> {
        let result = psql_db
            .query_one(
                "SELECT * FROM user_sessions 
                 WHERE   session_token = $1
                 AND expires_at > NOW()
                 LIMIT 1;",
                &[&session_token],
            )
            .await;

        match result {
            Ok(row) => AuthSession::from_row(&row),
            Err(e) => {
                eprintln!("{}", e);
                Err(e.into())
            }
        }
    }

    
    /*pub async fn delete_expired(
        public_address: Address,
        psql_db: &Database,
    ) -> Result<(), PostgresModelError> {
        let result = psql_db
            .query(
                "DELETE FROM user_sessions WHERE public_address = $1 AND expires_at <= NOW();",
                &[&DomainEthAddress(public_address)],
            )
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("{}", e);
                Err(e.into())
            }
        }
    }*/
}

pub async fn validate_session_token(
    session_token: &String,
    // wallet_public_address: &Address,
    app_state: &Data<AppState>,
) -> Option<AuthSession> {
    let existing =
        AuthSessionsModel::find_one(session_token.to_string(), &app_state.database).await;

    return existing.ok();
}
