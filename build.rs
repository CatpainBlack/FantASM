extern crate chrono;

use chrono::{Datelike, Local};

fn main() {
    let now = Local::now();
    println!("cargo:rustc-env=BUILD_DATE={:02}-{:02}-{:04}", now.day(), now.month(), now.year());
}