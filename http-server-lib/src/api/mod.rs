mod echo;
mod login;
mod verify_login;
mod logout;

use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;
use crate::api::echo::api_echo;
use crate::api::login::api_login;
use crate::api::logout::api_logout;
use crate::api::verify_login::api_verify_login;
use crate::headers::HttpHeaders;
use crate::http::{HttpRequest, HttpResponse, HttpResponseCode};
use crate::types::content_type::ContentType;

pub fn handle_api_request(request: &HttpRequest) -> HttpResponse {
    fn _inner(request: &HttpRequest) -> Result<HttpResponse, (HttpResponseCode, String)> {
        let e = (HttpResponseCode::not_found(), "API endpoint not found".into());
        macro_rules! e {
            ($e:expr) => { $e.ok_or(e.clone())? };
        }

        let relative_path = request.path.strip_prefix("/").unwrap_or(&request.path);
        let mut iter = relative_path.iter();
        if e!(iter.next()) != "api" {
            return Err(e);
        }
        match e!(e!(e!(iter.next()).to_str()).split("?").next()) {
            "echo" => api_echo(request),
            "login" => api_login(request),
            "verify-login" => api_verify_login(request),
            "logout" => api_logout(request),
            _ => Err(e),
        }
    }

    match _inner(request) {
        Ok(response) => response,
        Err((code, msg)) => HttpResponse::new(
            request.version,
            code,
            HttpHeaders::builder().content_type(ContentType::json()).build(),
            Some(format!(r#"{{"status": "error", "error": "{msg}"}}"#).into_bytes().into_boxed_slice())
        )
    }
}


pub struct QueryParameters {
    map: HashMap<String, String>,
}

impl QueryParameters {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    
    pub fn from_query_string(query_string: &str) -> Self {
        let query = if let Some((_, right)) = query_string.split_once("?") {
            right
        } else {
            query_string
        };
        let mut map = HashMap::new();
        for pair in query.split("&") {
            if let Some((key, value)) = pair.split_once("=") {
                map.insert(key.to_string(), value.to_string());
            }
        }
        Self { map }
    }
    
    pub fn get_param(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(String::as_str)
    }
}

impl FromStr for QueryParameters {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_query_string(s))
    }
}
