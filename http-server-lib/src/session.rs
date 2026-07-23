use core::net::SocketAddr;
use std::fs::read;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use crate::api;
use crate::error::RequestError;
use crate::headers::HttpHeaders;
use crate::http::{HttpRequest, HttpResponse, HttpResponseCode, HttpVersion};
use crate::types::content_type::ContentType;

pub struct HttpSession {
    id: Uuid,
    addr: SocketAddr,
    server_path: PathBuf,
}

impl HttpSession {
    pub fn new(id: Uuid, addr: SocketAddr, server_path: PathBuf) -> Self {
        Self { id, addr, server_path }
    }

    pub fn handle_request_bytes(&mut self, request_bytes: &[u8]) -> HttpResponse {
        match HttpRequest::try_from(request_bytes) {
            Ok(request) => self.handle_request(request),
            Err(error) => self.handle_error(error),
        }
    }

    pub fn handle_request(&mut self, request: HttpRequest) -> HttpResponse {
        let public_path = self.public_path();
        
        let relative_request_path = request.path.strip_prefix("/").unwrap_or(&request.path);
        
        if relative_request_path.starts_with(Path::new("api")) {
            return api::handle_api_request(&request);
        }
        
        let mut filepath = public_path.join(relative_request_path /* TODO: Don't expose the entire filesystem */);
        if filepath.is_dir() {
            filepath.push("index.html");
        }
        println!("Request path: {:?}", public_path.join(relative_request_path));
        let bytes = match read(&filepath) {
            Ok(bytes) => bytes.into_boxed_slice(),
            Err(_) => {
                filepath = public_path.join("error.html");
                match read(&filepath) {
                    Ok(bytes) => bytes.into_boxed_slice(),
                    Err(_) => "500 Internal Server Error".as_bytes().to_vec().into_boxed_slice(),
                }
            }
        };
        
        let content_type = ContentType::from_file_extension(
                filepath
                    .extension()
                    .map(|os_str| os_str.to_str().unwrap_or("html"))
                    .unwrap_or("html")
        );
        HttpResponse::new(
            request.version,
            HttpResponseCode::ok(),
            HttpHeaders::builder().content_type(content_type).build(),
            Some(bytes)
        )
    }

    pub fn handle_error(&mut self, error: RequestError) -> HttpResponse {
        let public_path = self.public_path();
        
        let bytes = match read(public_path.join("error.html")) {
            Ok(bytes) => bytes.into_boxed_slice(),
            Err(_) => format!("500 Internal Server Error: {error}").into_bytes().into_boxed_slice(),
        };
        HttpResponse::new(
            HttpVersion::Http11,
            HttpResponseCode::bad_request(),
            HttpHeaders::builder().content_type(ContentType::html()).build(),
            Some(bytes)
        )
    }

    fn public_path(&self) -> &Path {
        &self.server_path
    }
}
