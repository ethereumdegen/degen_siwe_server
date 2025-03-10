use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};

pub enum HeaderMapPreset {
    ApplicationJson,
    FormUrlEncoded,
    MultipartFormData,
    PlainText,
    BearerToken(String),
    //  BasicAuth(String, String),
    Custom(Vec<(String, String)>),
}

impl HeaderMapPreset {
    pub fn build(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        match self {
            Self::ApplicationJson => {
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
            }
            Self::FormUrlEncoded => {
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/x-www-form-urlencoded"),
                );
            }
            Self::MultipartFormData => {
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("multipart/form-data"),
                );
            }
            Self::PlainText => {
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
            }
            Self::BearerToken(token) => {
                let auth_value = format!("Bearer {}", token);
                if let Ok(header_value) = HeaderValue::from_str(&auth_value) {
                    headers.insert(reqwest::header::AUTHORIZATION, header_value);
                }
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            }

            Self::Custom(custom_headers) => {
                for (name, value) in custom_headers {
                    if let (Ok(header_name), Ok(header_value)) = (
                        name.parse::<reqwest::header::HeaderName>(),
                        HeaderValue::from_str(value),
                    ) {
                        headers.insert(header_name, header_value);
                    }
                }
            }
        }

        headers
    }

    pub fn with_extra_headers(self, extra_headers: Vec<(String, String)>) -> Self {
        match self {
            Self::Custom(mut headers) => {
                headers.extend(extra_headers);
                Self::Custom(headers)
            }
            _ => {
                let mut base_headers = match self {
                    Self::ApplicationJson => vec![
                        ("Content-Type".to_string(), "application/json".to_string()),
                        ("Accept".to_string(), "application/json".to_string()),
                    ],
                    Self::FormUrlEncoded => vec![(
                        "Content-Type".to_string(),
                        "application/x-www-form-urlencoded".to_string(),
                    )],
                    Self::MultipartFormData => vec![(
                        "Content-Type".to_string(),
                        "multipart/form-data".to_string(),
                    )],
                    Self::PlainText => vec![("Content-Type".to_string(), "text/plain".to_string())],
                    Self::BearerToken(token) => vec![
                        ("Authorization".to_string(), format!("Bearer {}", token)),
                        ("Content-Type".to_string(), "application/json".to_string()),
                    ],

                    Self::Custom(_) => unreachable!(),
                };

                base_headers.extend(extra_headers);
                Self::Custom(base_headers)
            }
        }
    }

    pub fn application_json_with_auth(token: &str) -> Self {
        Self::BearerToken(token.to_string())
    }

    pub fn merge(&self, other_headers: &HeaderMap) -> HeaderMap {
        let mut result = self.build();

        for (key, value) in other_headers.iter() {
            result.insert(key.clone(), value.clone());
        }

        result
    }
}

// Example usage:
//
// // Simple application/json headers
// let headers = HeaderMapPreset::ApplicationJson.build();
//
// // Bearer token authentication with JSON content type
// let headers = HeaderMapPreset::BearerToken("your-token-here".to_string()).build();
//
// // Custom headers
// let custom_headers = vec![
//     ("X-API-Key".to_string(), "your-api-key".to_string()),
//     ("User-Agent".to_string(), "MyApp/1.0".to_string())
// ];
// let headers = HeaderMapPreset::Custom(custom_headers).build();
//
// // Adding extra headers to a preset
// let headers = HeaderMapPreset::ApplicationJson
//     .with_extra_headers(vec![
//         ("X-Request-ID".to_string(), "123456".to_string())
//     ])
//     .build();
