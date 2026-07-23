use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use crate::headers::HttpHeaders;
use crate::error::{RequestError, ResponseError};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Head,
    Put,
    Delete,
    Patch
}

impl FromStr for HttpMethod {
    type Err = RequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "HEAD" => Ok(HttpMethod::Head),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "PATCH" => Ok(HttpMethod::Patch),
            _ => Err(RequestError::InvalidRequestType)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HttpVersion {
    Http10,
    Http11,
}

impl FromStr for HttpVersion {
    type Err = RequestError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(HttpVersion::Http10),
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            _ => Err(RequestError::UnsupportedHttpVersion)
        }
    }
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HttpVersion::Http10 => "HTTP/1.0".to_string(),
            HttpVersion::Http11 => "HTTP/1.1".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest<'a> {
    pub method: HttpMethod,
    pub path: PathBuf,
    pub version: HttpVersion,
    pub headers: HttpHeaders,
    pub body: Option<&'a [u8]>
}

impl<'a> TryFrom<&'a [u8]> for HttpRequest<'a> {
    type Error = RequestError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let index_body_start = bytes
            .windows(4)
            .position(|window| window == b"\r\n\r\n");

        let header_bytes = &bytes[..index_body_start.unwrap_or(bytes.len())];
        let header = str::from_utf8(header_bytes)?;
        let mut lines = header.lines();

        let mut first_line = lines
            .next()
            .ok_or(RequestError::MalformedRequest)?
            .splitn(3, ' ');
        let method = HttpMethod::from_str(
            first_line
                .next()
                .ok_or(RequestError::MalformedRequest)?
        )?;
        let path = PathBuf::from(
            first_line
                .next()
                .ok_or(RequestError::MalformedRequest)?
        );
        let version = HttpVersion::from_str(
            first_line
                .next()
                .ok_or(RequestError::MalformedRequest)?
        )?;

        let headers = HttpHeaders::from(lines);

        let mut body = None;

        if let Some(index_body_start_with_line_terminators) = index_body_start {
            let index_body_start = index_body_start_with_line_terminators + 4;
            if index_body_start < bytes.len() {
                body = Some(&bytes[index_body_start..]);
            }
        }

        Ok(Self {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HttpResponseCode {
    Informational(u8),
    Success(u8),
    Redirection(u8),
    ClientError(u8),
    ServerError(u8),
}

impl HttpResponseCode {
    pub fn ok() -> Self { Self::Success(0) }
    pub fn bad_request() -> Self { Self::ClientError(0) }
    pub fn forbidden() -> Self { Self::ClientError(3) }
    pub fn not_found() -> Self { Self::ClientError(4) }
    pub fn method_not_allowed() -> Self { Self::ClientError(5) }

    pub fn to_number(self) -> u16 {
        self.into()
    }

    pub fn message(&self) -> Option<&'static str> {
        match self {
            Self::Informational(code) => match code {
                0 => Some("Continue"),
                1 => Some("Switching Protocols"),
                2 => Some("Processing"),
                3 => Some("Early Hints"),
                _ => None
            },
            Self::Success(code) => match code {
                0 => Some("OK"),
                1 => Some("Created"),
                2 => Some("Accepted"),
                3 => Some("Non-Authoritative Information"),
                4 => Some("No Content"),
                5 => Some("Reset Content"),
                6 => Some("Partial Content"),
                7 => Some("Multi-Status"),
                8 => Some("Already Reported"),
                26 => Some("IM Used"),
                _ => None
            },
            Self::Redirection(code) => match code {
                0 => Some("Multiple Choices"),
                1 => Some("Moved Permanently"),
                2 => Some("Found"),
                3 => Some("See Other"),
                4 => Some("Not Modified"),
                7 => Some("Temporary Redirect"),
                8 => Some("Permanent Redirect"),
                _ => None
            },
            Self::ClientError(code) => match code {
                0 => Some("Bad Request"),
                1 => Some("Unauthorized"),
                2 => Some("Payment Required"),
                3 => Some("Forbidden"),
                4 => Some("Not Found"),
                5 => Some("Method Not Allowed"),
                6 => Some("Not Acceptable"),
                7 => Some("Proxy Authentication Required"),
                8 => Some("Request Timeout"),
                9 => Some("Conflict"),
                10 => Some("Gone"),
                11 => Some("Length Required"),
                12 => Some("Precondition Failed"),
                13 => Some("Content Too Large"),
                14 => Some("URI Too Long"),
                15 => Some("Unsupported Media Type"),
                16 => Some("Range Not Satisfiable"),
                17 => Some("Expectation Failed"),
                18 => Some("I'm a teapot"),
                21 => Some("Misdirected Request"),
                22 => Some("Unprocessable Content"),
                23 => Some("Locked"),
                24 => Some("Failed Dependency"),
                25 => Some("Too Early"),
                26 => Some("Upgrade Required"),
                28 => Some("Precondition Required"),
                29 => Some("Too Many Requests"),
                31 => Some("Request Header Fields Too Large"),
                51 => Some("Unavailable For Legal Reasons"),
                _ => None
            },
            Self::ServerError(code) => match code {
                0 => Some("Internal Server Error"),
                1 => Some("Not Implemented"),
                2 => Some("Bad Gateway"),
                3 => Some("Service Unavailable"),
                4 => Some("Gateway Timeout"),
                5 => Some("HTTP Version Not Supported"),
                6 => Some("Variant Also Negotiates"),
                7 => Some("Insufficient Storage"),
                8 => Some("Loop Detected"),
                10 => Some("Not Extended"),
                11 => Some("Network Authentication Required"),
                _ => None
            }
        }
    }
}

impl TryFrom<u16> for HttpResponseCode {
    type Error = ResponseError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        match code {
            100..=199 => Ok(Self::Informational((code - 100) as u8)),
            200..=299 => Ok(Self::Success((code - 200) as u8)),
            300..=399 => Ok(Self::Redirection((code - 300) as u8)),
            400..=499 => Ok(Self::ClientError((code - 400) as u8)),
            500..=599 => Ok(Self::ServerError((code - 500) as u8)),
            code => Err(Self::Error::InvalidStatusCode(code))
        }
    }
}

impl Into<u16> for HttpResponseCode {
    fn into(self) -> u16 {
        match self {
            Self::Informational(code) => 100 + (code as u16),
            Self::Success(code) => 200 + (code as u16),
            Self::Redirection(code) => 300 + (code as u16),
            Self::ClientError(code) => 400 + (code as u16),
            Self::ServerError(code) => 500 + (code as u16),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub response_code: HttpResponseCode,
    pub headers: HttpHeaders,
    pub body: Option<Box<[u8]>>,
}

impl HttpResponse {
    pub fn new(
        version: HttpVersion, 
        response_code: HttpResponseCode, 
        mut headers: HttpHeaders,
        body: Option<Box<[u8]>>
    ) -> Self {
        if let Some(bytes) = &body {
            headers.add_header("Content-Length".to_string(), bytes.len().to_string());
        }
        Self { version, response_code, headers, body }
    }
    
    pub fn into_bytes(self) -> Vec<u8> {
        self.into()
    }
}

impl Into<Vec<u8>> for HttpResponse {
    fn into(self) -> Vec<u8> {
        let response = format!(
            "{} {} {}\r\n{}\r\n",
            self.version,
            self.response_code.to_number(),
            self.response_code.message().unwrap_or("Unknown"),
            self.headers
        );
        let mut response_bytes = response.into_bytes();
        if let Some(body) = self.body {
            response_bytes.append(&mut body.into_vec());
        }
        response_bytes
    }
}
