pub struct ZXAscii {}

impl ZXAscii {
    pub fn zx_safe(string: &str) -> String {
        let x = string.to_string()
            .replace("£", "`")
            .replace("©", "\u{007F}")
            .replace("↑", "^");
        //println!("{} - {:02X?} ", x, x.as_bytes());
        x
    }
}