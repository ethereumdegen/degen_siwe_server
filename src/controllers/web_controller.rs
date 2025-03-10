use actix_web::{web::ServiceConfig, Responder};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use utoipa::ToSchema;

pub trait WebController {
    fn config(cfg: &mut ServiceConfig);
}

#[derive(Debug)]
pub struct WebControllerSuccessResponse;
impl Serialize for WebControllerSuccessResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("WebControllerSuccessResponse", 1)?;
        state.serialize_field("success", &true)?;
        state.end()
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct WebControllerErrorResponse {
    error_message: String,
}

impl WebControllerErrorResponse {
    pub fn new(input: impl ToString) -> Self {
        Self {
            error_message: input.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
pub struct AuthSessionOutput {
    pub public_address: String,
    pub session_token: String,
    pub expires_at: i64,
}


/*
#[derive(Deserialize, Serialize, ToSchema)]
pub struct AuthResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}
*/





#[derive(Deserialize, Serialize, ToSchema)]
pub struct SuccessResponse<T> {

    pub success: bool,
    pub data: T 

}

impl<T> SuccessResponse<T> {

    pub fn new( data : T ) -> Self {
        Self {

            success:true,
            data
        }
    }
}
 


#[derive(Deserialize, Serialize, ToSchema)]
pub struct ErrorResponse {
     pub success: bool,
     pub error: String  
}



impl ErrorResponse {

    pub fn new( error : impl ToString ) -> Self {
        Self {

            success:false,
            error: error.to_string()
        }
    }
}
 

/*
impl<T> Responder for SuccessResponse<T> {
    type T;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        todo!()
    }

    
}



impl Responder for ErrorResponse {
     type String;
      fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        todo!()
    }
}*/