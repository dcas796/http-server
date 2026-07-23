use std::ffi::OsStr;
use crate::api::QueryParameters;
use crate::headers::HttpHeaders;
use crate::http::{HttpMethod, HttpRequest, HttpResponse, HttpResponseCode};
use crate::types::content_type::ContentType;

pub fn api_echo(request: &HttpRequest) -> Result<HttpResponse, (HttpResponseCode, String)> {
    if request.method != HttpMethod::Get {
        return Err((HttpResponseCode::method_not_allowed(), "Only GET requests allowed".into()));
    }

    let query = request.path
        .file_name()
        .unwrap_or(OsStr::new("test"))
        .to_str()
        .unwrap_or("test");
    let parameters = QueryParameters::from_query_string(query);
    let message = parameters.get_param("m").unwrap_or("");
    Ok(HttpResponse::new(
        request.version,
        HttpResponseCode::ok(),
        HttpHeaders::builder().content_type(ContentType::json()).build(),
        Some(format!(r#"{{"status": "success", "message": "{}"}}"#, message).into_bytes().into_boxed_slice()),
    ))
}
