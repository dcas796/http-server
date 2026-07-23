use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;
use crate::types::content_type::ContentType;
use crate::types::cookie::Cookie;

#[derive(Debug, Clone)]
pub struct HttpHeaders {
    headers: HashMap<String, String>,
}

macro_rules! get_header_method {
    ($name:ident, $key:expr, $t:ty) => {
        pub fn $name(&self) -> Option<$t> {
            self.get_header($key).map(Into::into)
        }
    };
}

impl HttpHeaders {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn builder() -> HttpHeadersBuilder {
        HttpHeadersBuilder::new()
    }

    pub fn get_header<'a>(&'a self, key: &'_ str) -> Option<&'a str> {
        self.headers.get(key).map(String::as_str)
    }
    
    get_header_method!(get_cookie, "Cookie", Cookie);

    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn remove_header(&mut self, key: &str) -> Option<String> {
        self.headers.remove(key)
    }
}

impl FromStr for HttpHeaders {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.lines()))
    }
}

impl<'a, T: IntoIterator<Item = &'a str>> From<T> for HttpHeaders {
    fn from(lines: T) -> Self {
        let mut headers = HashMap::new();
        for line in lines {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }
        Self { headers }
    }
}

impl Display for HttpHeaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value)?;
        }
        Ok(())
    }
}


macro_rules! builder_method {
    ($name:ident, $key:expr, $value_type:ty) => {
        pub fn $name(self, value: $value_type) -> Self {
            self.header($key, value)
        }
    };
}

pub struct HttpHeadersBuilder {
    headers: HashMap<String, String>,
}

impl HttpHeadersBuilder {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn build(self) -> HttpHeaders {
        self.into()
    }

    pub fn header(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    builder_method!(content_type, "Content-Type", ContentType);
    builder_method!(set_cookie, "Set-Cookie", String);
}

impl Into<HttpHeaders> for HttpHeadersBuilder {
    fn into(self) -> HttpHeaders {
        HttpHeaders {
            headers: self.headers,
        }
    }
}
