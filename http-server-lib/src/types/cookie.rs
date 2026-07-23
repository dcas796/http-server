use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cookie {
    list: HashMap<String, String>,
}

impl Cookie {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    pub fn from_cookie_str(s: &str) -> Self {
        let mut list = HashMap::new();
        for mut kv in s.split(";") {
            kv = kv.trim();
            if let Some((key, value)) = kv.split_once("=") && !key.is_empty() {
                list.insert(key.to_string(), value.to_string());
            }
        }
        Self {
            list
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&str> {
        self.list.get(key).map(String::as_str)
    }
}

impl From<&str> for Cookie {
    fn from(s: &str) -> Self {
        Self::from_cookie_str(s)
    }
}

impl FromStr for Cookie {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_cookie_str(s))
    }
}
