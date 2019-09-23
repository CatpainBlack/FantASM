use regex::Regex;

lazy_static! {
static ref HEX: Regex = Regex::new(r"^0(?P<h1>[0-9a-fA-F]+)[hH]$|^0x(?P<h2>[0-9a-fA-F]+)$|^\$(?P<h3>[0-9a-fA-F]+)$").unwrap();
static ref BIN: Regex = Regex::new(r"^(?P<b1>[0-1]+)b$|^(0b|%)(?P<b2>[0-1]+)$").unwrap();
static ref DEC: Regex = Regex::new(r"^\d+$").unwrap();
}

pub trait NumberParser {
    fn to_number(&self) -> Option<i64>;
}

impl NumberParser for String {
    fn to_number(&self) -> Option<i64> {
        if let Some(captures) = HEX.captures(&self) {
            for name in HEX.capture_names() {
                if let Some(n) = name {
                    if let Some(val) = captures.name(n) {
                        return Some(i64::from_str_radix(val.as_str(), 16).unwrap());
                    }
                }
            }
        }

        if let Some(captures) = BIN.captures(&self) {
            for name in BIN.capture_names() {
                if let Some(n) = name {
                    if let Some(val) = captures.name(n) {
                        return Some(i64::from_str_radix(val.as_str(), 2).unwrap());
                    }
                }
            }
        }

        if DEC.is_match(&self) {
            return Some(i64::from_str_radix(&self, 10).unwrap());
        }
        None
    }
}