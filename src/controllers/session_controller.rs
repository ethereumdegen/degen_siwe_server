/*


Web3 wallet signature authenticatiaon !



curl -X POST http://localhost:8080/api/session/generate_challenge    -H "Content-Type: application/json"   -d '{"public_address": "0x810E096DDa9ae3Ae2b55a9c45068F9FE8eeea6db"}'
 

curl -X POST https://api.inkreel.com/api/session/generate_challenge      -H "Content-Type: application/json"      -d '{"public_address": "0x810E096DDa9ae3Ae2b55a9c45068F9FE8eeea6db"}'


curl -X POST https://coral-app-feoar.ondigitalocean.app/api/session/generate_challenge
   -H "Content-Type: application/json"
      -d '{"public_address": "0x810E096DDa9ae3Ae2b55a9c45068F9FE8eeea6db"}'


curl -X POST http://localhost:8080/api/session/validate_auth     -H "Content-Type: application/json"   -d '{"challenge" : "Signing in to inkreel as 0x810e096dda9ae3ae2b55a9c45068f9fe8eeea6db at 1740165511" , "public_address": "0x810E096DDa9ae3Ae2b55a9c45068F9FE8eeea6db" , "signature": "0x71706a8de0b3e5a42a4ffd1ab7a6ce5c77ed16cea9cb251641b8bcce669cce8d5fd0587b655debff8b62d406d2ca7c0d961f2c36f2c7d3f7a61251812ba3b2331c"}'

curl -X POST https://coral-app-feoar.ondigitalocean.app/api/session/validate_auth     -H "Content-Type: application/json"   -d '{"challenge" : "Signing in to inkreel as 0x810e096dda9ae3ae2b55a9c45068f9fe8eeea6db at 1740165511" , "public_address": "0x810E096DDa9ae3Ae2b55a9c45068F9FE8eeea6db" , "signature": "0x71706a8de0b3e5a42a4ffd1ab7a6ce5c77ed16cea9cb251641b8bcce669cce8d5fd0587b655debff8b62d406d2ca7c0d961f2c36f2c7d3f7a61251812ba3b2331c"}'
 

CREATE TABLE challenge_tokens (
   id SERIAL PRIMARY KEY,
    public_address VARCHAR(255) NOT NULL UNIQUE,
    challenge TEXT NOT NULL,
      created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_sessions (
    id SERIAL PRIMARY KEY,
    public_address VARCHAR(255) NOT NULL,
    session_token VARCHAR(255)  NOT NULL,
     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);




*/

use actix_web::web::{Data, ServiceConfig};
use actix_web::{web, HttpResponse, Responder};
use degen_siwe_server::db::postgres::models::auth_challenges_model::{
    AuthChallenge, AuthChallengesModel,
};
use degen_siwe_server::db::postgres::models::auth_sessions_model::{
    AuthSession, AuthSessionsModel,
};
use ethers::types::Address;
use serde::{Deserialize, Serialize};

use crate::controllers::web_controller::AuthResponse;

use super::web_controller::WebController;
use degen_siwe_server::app_state::AppState;

use crate::controllers::web_controller::AuthSessionOutput;

pub struct SessionController {}

impl SessionController {}

impl WebController for SessionController {
    fn config(cfg: &mut ServiceConfig) {
        cfg.service(
            web::scope("/api/session")
                // Add your routes here, e.g.,
                .route("/generate_challenge", web::post().to(generate_challenge))
                .route("/validate_auth", web::post().to(validate_authentication)),
        );
    }
}

#[derive(Deserialize, Serialize)]
struct ChallengeResponse {
    success: bool,
    challenge: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct GenerateChallengeRequest {
    public_address: String,
}

#[derive(Deserialize)]
struct ValidateAuthRequest {
    public_address: String,
    challenge: String,
    signature: String,
}

async fn generate_challenge(
    req: web::Json<GenerateChallengeRequest>,
    app_state: Data<AppState>,
) -> impl Responder {
    let public_address_str = req.public_address.trim().to_lowercase();

    println!(" public_address_str {} ", public_address_str);

    let Ok(public_address) = public_address_str.parse::<Address>() else {
        return HttpResponse::BadRequest().json(ChallengeResponse {
            success: false,
            challenge: None,
            error: Some("Invalid public address".to_string()),
        });
    };

    let service_name = & app_state.app_config.service_name; 

    let new_challenge = AuthChallenge::new(public_address,   service_name);

    let inserted =
        AuthChallengesModel::insert_one(new_challenge.clone(), &app_state.database).await;

    match inserted {
        Ok(_) => HttpResponse::Ok().json(ChallengeResponse {
            success: true,
            challenge: Some(new_challenge.challenge.clone()),
            error: None,
        }),
        Err(_) => HttpResponse::InternalServerError().json(ChallengeResponse {
            success: false,
            challenge: None,
            error: Some("Database error".to_string()),
        }),
    }
}

async fn validate_authentication(
    req: web::Json<ValidateAuthRequest>,
    app_state: Data<AppState>,
) -> impl Responder {
    let public_address_str = req.public_address.trim().to_lowercase();
    let challenge = &req.challenge;
    let signature = &req.signature;

   // println!(" public_address_str {} ", public_address_str);

    let Ok(public_address) = public_address_str.parse::<Address>() else {
        return HttpResponse::BadRequest().json(ChallengeResponse {
            success: false,
            challenge: None,
            error: Some("Invalid public address".to_string()),
        });
    };

    let challenge_record =
        AuthChallengesModel::find_one(&public_address, challenge, &app_state.database).await;

    

    if let Ok(record) = challenge_record {
        if &record.challenge != challenge {
            return HttpResponse::Unauthorized().json(AuthResponse::<String> {
                success: false,
                data: None,
                error: Some("Invalid challenge".to_string()),
            });
        }
    } else {
        return HttpResponse::Unauthorized().json(AuthResponse::<String> {
            success: false,
            data: None,
            error: Some("No active challenge found".to_string()),
        });
    }

    // Verify signature
    let recovered_address = recover_address(challenge, signature);

    if recovered_address.as_deref() != Some(public_address_str.as_str()) {
        return HttpResponse::Unauthorized().json(AuthResponse::<String> {
            success: false,
            data: None,
            error: Some("Invalid signature".to_string()),
        });
    }

    let expires_in_days = 1;

    let new_user_session = AuthSession::new(public_address, expires_in_days);

    let inserted =
        AuthSessionsModel::insert_one(new_user_session.clone(), &app_state.database).await;

    

    match inserted {
        Ok(_) => {
            let session_data_output = AuthSessionOutput {
                public_address: new_user_session.public_address.to_string_full(),
                session_token: new_user_session.session_token,
                expires_at: new_user_session.expires_at.timestamp(),
            };

            HttpResponse::Ok().json(AuthResponse {
                success: true,
                data: Some(session_data_output),
                error: None,
            })
        }
        Err(_) => HttpResponse::InternalServerError().json(AuthResponse::<String> {
            success: false,
            data: None,
            error: Some("Database error".to_string()),
        }),
    }
}

fn recover_address(msg: &str, signature: &str) -> Option<String> {
    use ethers::core::types::Signature;
    use ethers::utils::hash_message;

    let sig_bytes = hex::decode(signature.strip_prefix("0x").unwrap_or(signature)).ok()?;
    let sig = Signature::try_from(sig_bytes.as_slice()).ok()?;

    let msg_hash = hash_message(msg);
    let recovered = sig.recover(msg_hash).ok()?;

    Some(format!("{:?}", recovered))
}
 