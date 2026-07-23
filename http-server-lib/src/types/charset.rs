use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Charset {
    Utf8,
    Other(String),
}

impl Display for Charset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Charset::Utf8 => "UTF-8".to_string(),
            Charset::Other(s) => s.clone(),
        };
        write!(f, "{}", str)
    }
}
