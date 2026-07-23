use crate::headers::HttpHeaders;
use crate::http::{HttpRequest, HttpResponse, HttpResponseCode};
use crate::types::content_type::ContentType;

pub fn api_login(request: &HttpRequest) -> Result<HttpResponse, (HttpResponseCode, String)> {
    if request.method != crate::http::HttpMethod::Post {
        return Err((HttpResponseCode::method_not_allowed(), "Only POST requests allowed".into()));
    }

    let body = str::from_utf8(
        request.body
            .ok_or((HttpResponseCode::bad_request(), "Missing request body".into()))?
    ).map_err(|_| (HttpResponseCode::bad_request(), "Invalid UTF-8 in request body".into()))?;

    let mut username = None;
    let mut password = None;

    for line in body.lines() {
        if let Some((key, value)) = line.split_once('=') {
            match key {
                "username" => username = Some(value),
                "password" => password = Some(value),
                _ => {}
            }
        }
    }

    let username = username.ok_or((HttpResponseCode::bad_request(), "Missing username".into()))?;
    let password = password.ok_or((HttpResponseCode::bad_request(), "Missing password".into()))?;

    // TODO: this is a very bad thing to do
    let cookie = sha256::digest(format!("{}\0{}", username, password));

    Ok(HttpResponse::new(
        request.version,
        HttpResponseCode::ok(),
        HttpHeaders::builder()
            .content_type(ContentType::json())
            .set_cookie(format!("session={}; HttpOnly; Path=/", cookie))
            .build(),
        Some(r#"{"status": "success", "message": "Logged in successfully"}"#.as_bytes().to_vec().into_boxed_slice()),
    ))
}
