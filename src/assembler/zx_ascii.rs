pub struct ZXAscii {}

impl ZXAscii {
    pub fn zx_safe(src: &str) -> String {
        src.to_string()
            .replace("£", "`")
            .replace("©", "\u{007F}")
            .replace("↑", "^")
    }
}