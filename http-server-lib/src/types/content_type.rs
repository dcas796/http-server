use std::fmt::Display;
use crate::types::charset::Charset;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentType {
    content_type: String,
    charset: Option<Charset>,
}

impl ContentType {
    pub fn new(content_type: String, charset: Option<Charset>) -> Self {
        Self { content_type, charset }
    }
    
    pub fn plain() -> Self { Self::new("text/plain".to_string(), None) }
    pub fn html() -> Self { Self::new("text/html".to_string(), Some(Charset::Utf8)) }
    pub fn css() -> Self { Self::new("text/css".to_string(), Some(Charset::Utf8)) }
    pub fn javascript() -> Self { Self::new("application/javascript".to_string(), Some(Charset::Utf8)) }
    pub fn json() -> Self { Self::new("application/json".to_string(), Some(Charset::Utf8)) }
    pub fn png() -> Self { Self::new("image/png".to_string(), None) }
    
    pub fn from_file_extension(extension: &str) -> Self {
        match extension {
            "html" => Self::html(),
            "css" => Self::css(),
            "js" => Self::javascript(),
            "png" => Self::png(),
            _ => Self::plain(),
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content_type)?;
        if let Some(charset) = &self.charset {
            write!(f, "; charset={}", charset)?;
        }
        Ok(())
    }
}
