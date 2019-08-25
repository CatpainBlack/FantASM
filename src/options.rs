extern crate argparse;

use std::path::Path;
use argparse::{ArgumentParser, Store, StoreTrue};

#[derive(Default, Debug)]
pub struct Options {
    pub source: String,
    pub output: String,
    pub z80n: bool,
    pub verbose: bool,
    pub debug: bool,
    pub no_logo: bool,
    pub c_spect: bool,
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

            parser.refer(&mut options.z80n)
                .add_option(&["--z80n"], StoreTrue, "Enable Z80n (ZX Next) cpu extensions");

            parser.refer(&mut options.c_spect)
                .add_option(&["--cspect"], StoreTrue, "Enable cspect \"exit\" and \"break\" instructions");

            parser.refer(&mut options.no_logo)
                .add_option(&["-n", "--nologo"], StoreTrue, "Do no display the program name and version");

            parser.refer(&mut options.debug)
                .add_option(&["--debug"], StoreTrue, "Enable assembler information dump");

            parser.refer(&mut options.verbose)
                .add_option(&["-v", "--verbose"], StoreTrue, "Enable verbose output");

            parser.parse_args_or_exit();
        }

        if !Path::new(&options.source).exists() {
            return Err(format!("Source file: {} - does not exist", options.source));
        }

        Ok(options)
    }
}