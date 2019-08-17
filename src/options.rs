extern crate argparse;

use std::path::Path;

use argparse::{ArgumentParser, Store, StoreTrue};

#[derive(Default, Debug)]
pub struct Options {
    pub source: String,
    pub output: String,
    pub verbose: bool,
}

impl Options {
    pub fn parse() -> Result<Options, String> {
        let mut options = Options::default();
        {
            let mut parser = ArgumentParser::new();
            parser.set_description("phantasm - a Z80 assembler");

            parser.refer(&mut options.source)
                .add_argument("input", Store, "Source file")
                .required();

            parser.refer(&mut options.output)
                .add_argument("output", Store, "Output file")
                .required();

            parser.refer(&mut options.verbose)
                .add_option(&["-v", "--verbose"], StoreTrue, "Enable verbose output");

            parser.parse_args_or_exit();
        }

        if !Path::new(&options.source).exists() {
            return Err(format!("Source file: {} - does not exist", options.source))
        }

        Ok(options)
    }
}