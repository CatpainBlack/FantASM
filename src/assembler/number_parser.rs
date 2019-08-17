use regex::Regex;

pub trait NumberParser {
    fn to_number(&self) -> Option<i64>;
}

impl NumberParser for String {
    fn to_number(&self) -> Option<i64> {
        let hex = Regex::new(r"^0(?P<h1>[0-9a-fA-F]+)[hH]$|0x(?P<h2>[0-9a-fA-F]+)$|^\$(?P<h3>[0-9a-fA-F]+)$").unwrap();
        let bin = Regex::new(r"^(?P<b1>[0-1]+)b$|^(0b|%)(?P<b2>[0-1]+)$").unwrap();
        let dec = Regex::new(r"^\d+$").unwrap();

        if let Some(captures) = hex.captures(&self) {
            for name in hex.capture_names() {
                if let Some(n) = name {
                    if let Some(val) = captures.name(n) {
                        return Some(i64::from_str_radix(val.as_str(), 16).unwrap());
                    }
                }
            }
        }

        if let Some(captures) = bin.captures(&self) {
            for name in bin.capture_names() {
                if let Some(n) = name {
                    if let Some(val) = captures.name(n) {
                        return Some(i64::from_str_radix(val.as_str(), 2).unwrap());
                    }
                }
            }
        }

        if dec.is_match(&self) {
            return Some(i64::from_str_radix(&self, 10).unwrap());
        }
        None
    }
}