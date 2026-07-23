use crate::headers::HttpHeaders;
use crate::http::{HttpMethod, HttpRequest, HttpResponse, HttpResponseCode};
use crate::types::content_type::ContentType;

pub fn api_verify_login(request: &HttpRequest) -> Result<HttpResponse, (HttpResponseCode, String)> {
    if request.method != HttpMethod::Get {
        return Err((HttpResponseCode::method_not_allowed(), "Only GET requests allowed".into()));
    }

    let not_logged_in = (HttpResponseCode::forbidden(), "You are not logged in".into());
    let cookie = request.headers
        .get_cookie()
        .ok_or(not_logged_in.clone())?;
    let session = cookie
        .get_value("session")
        .ok_or(not_logged_in.clone())?;
    
    if session.is_empty() {
        return Err(not_logged_in);
    }
    
    Ok(HttpResponse::new(
        request.version,
        HttpResponseCode::ok(),
        HttpHeaders::builder().content_type(ContentType::json()).build(),
        Some(r#"{"status": "success", "message": "Logged in successfully"}"#.as_bytes().to_vec().into_boxed_slice()),
    ))
}
