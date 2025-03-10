use reqwest::header::HeaderMap;
use reqwest::{Error, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub enum EndpointType {
    POST,
    GET,
}

pub trait IntoHttpRequest {
    fn get_url(&self) -> String;
    fn get_data(&self) -> serde_json::Value;
    fn get_headers(&self) -> Option<HeaderMap>;
    fn get_endpoint_type(&self) -> EndpointType;
}

pub struct EndpointUrlAndData {
    pub url: String,
    pub data: serde_json::Value,
    pub headers: HeaderMap,
    pub endpoint_type: EndpointType,
}

impl IntoHttpRequest for EndpointUrlAndData {
    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn get_data(&self) -> serde_json::Value {
        self.data.clone()
    }

    fn get_headers(&self) -> Option<HeaderMap> {
        Some(self.headers.clone())
    }

    fn get_endpoint_type(&self) -> EndpointType {
        match self.endpoint_type {
            EndpointType::POST => EndpointType::POST,
            EndpointType::GET => EndpointType::GET,
        }
    }
}

// Helper functions for making HTTP requests
async fn post_request(url: &str, data: &Value, headers: &HeaderMap) -> Result<Response, Error> {
    let client = reqwest::Client::new();

    println!("sending post {:?} {:?} {:?}", url, data, headers);

    client
        .post(url)
        .headers(headers.clone())
        .json(data)
        .send()
        .await
}

async fn get_request(url: &str, data: &Value, headers: &HeaderMap) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client
        .get(url)
        .headers(headers.clone())
        .query(&data)
        .send()
        .await
}

impl EndpointUrlAndData {
    pub async fn perform_req(&self) -> Result<Response, Error> {
        let url = &self.url;
        let data = &self.data;
        let header_map = &self.headers;
        match self.endpoint_type {
            EndpointType::POST => post_request(url, data, header_map).await,
            EndpointType::GET => get_request(url, data, header_map).await,
        }
    }

    pub async fn perform_req_typed<T>(&self) -> Result<Option<T>, Error>
    where
        T: DeserializeOwned,
    {
        let url = &self.url;
        let data = &self.data;
        let header_map = &self.headers;

        let response_result = match self.endpoint_type {
            EndpointType::POST => post_request(url, data, header_map).await,
            EndpointType::GET => get_request(url, data, header_map).await,
        };

        match response_result {
            Ok(res) => {
                // Try to deserialize the response body into type T
                match res.json::<T>().await {
                    Ok(typed_response) => Ok(Some(typed_response)),
                    Err(_) => Ok(None), // Deserialization failed, return None
                }
            }
            Err(e) => Err(e),
        }
    }
}

// Generic versions of perform_req and perform_req_typed that work with any IntoHttpRequest
pub async fn perform_req<T: IntoHttpRequest>(request: &T) -> Result<Response, Error> {
    let url = request.get_url();
    let data = request.get_data();
    let headers = request.get_headers().unwrap_or_else(|| HeaderMap::new());

    match request.get_endpoint_type() {
        EndpointType::POST => post_request(&url, &data, &headers).await,
        EndpointType::GET => get_request(&url, &data, &headers).await,
    }
}

pub async fn perform_req_typed<T, R>(request: &T) -> Result<Option<R>, Error>
where
    T: IntoHttpRequest,
    R: DeserializeOwned,
{
    let url = request.get_url();
    let data = request.get_data();
    let headers = request.get_headers().unwrap_or_else(|| HeaderMap::new());

    let response_result = match request.get_endpoint_type() {
        EndpointType::POST => post_request(&url, &data, &headers).await,
        EndpointType::GET => get_request(&url, &data, &headers).await,
    };

    match response_result {
        Ok(res) => {
            // Try to deserialize the response body into type R
            match res.json::<R>().await {
                Ok(typed_response) => Ok(Some(typed_response)),
                Err(_) => Ok(None), // Deserialization failed, return None
            }
        }
        Err(e) => Err(e),
    }
}

/*
// Example usage with another struct implementing IntoHttpRequest
#[derive(Clone)]
pub struct ApiRequest {
    base_url: String,
    endpoint: String,
    payload: serde_json::Value,
    custom_headers: Option<HeaderMap>,
    method: EndpointType,
}

impl ApiRequest {
    pub fn new(base_url: &str, endpoint: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            endpoint: endpoint.to_string(),
            payload: serde_json::json!({}),
            custom_headers: None,
            method: EndpointType::GET,
        }
    }

    pub fn with_payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = payload;
        self
    }

    pub fn with_method(mut self, method: EndpointType) -> Self {
        self.method = method;
        self
    }

    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.custom_headers = Some(headers);
        self
    }

    // Convenience method to use the generic functions
    pub async fn send(&self) -> Result<Response, Error> {
        perform_req(self).await
    }

    pub async fn send_typed<R: DeserializeOwned>(&self) -> Result<Option<R>, Error> {
        perform_req_typed::<_, R>(self).await
    }
}

impl IntoHttpRequest for ApiRequest {
    fn get_url(&self) -> String {
        format!("{}{}", self.base_url, self.endpoint)
    }

    fn get_data(&self) -> serde_json::Value {
        self.payload.clone()
    }

    fn get_headers(&self) -> Option<HeaderMap> {
        self.custom_headers.clone()
    }

    fn get_endpoint_type(&self) -> EndpointType {
        match self.method {
            EndpointType::POST => EndpointType::POST,
            EndpointType::GET => EndpointType::GET,
        }
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TestResponse {
        message: String,
    }

    #[tokio::test]
    async fn test_endpoint_url_and_data() {
        // This is just a compile-time test to ensure the functions work with the trait
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let endpoint = EndpointUrlAndData {
            url: "https://example.com/api".to_string(),
            data: serde_json::json!({"key": "value"}),
            headers,
            endpoint_type: EndpointType::POST,
        };

        // These would make actual network requests, so we don't run them in tests
        // let _response = endpoint.perform_req().await;
        // let _typed_response: Result<Option<TestResponse>, Error> = endpoint.perform_req_typed().await;

        // But we can test using the generic functions
        // let _response = perform_req(&endpoint).await;
        // let _typed_response: Result<Option<TestResponse>, Error> = perform_req_typed(&endpoint).await;
    }

    #[tokio::test]
    async fn test_api_request() {
        // Test the ApiRequest implementation
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer token".parse().unwrap());

        let request = ApiRequest::new("https://example.com", "/api/test")
            .with_payload(serde_json::json!({"test": true}))
            .with_method(EndpointType::POST)
            .with_headers(headers);

        // These would make actual network requests, so we don't run them in tests
        // let _response = request.send().await;
        // let _typed_response: Result<Option<TestResponse>, Error> = request.send_typed().await;

        // But we can test using the generic functions
        // let _response = perform_req(&request).await;
        // let _typed_response: Result<Option<TestResponse>, Error> = perform_req_typed(&request).await;
    }
}
