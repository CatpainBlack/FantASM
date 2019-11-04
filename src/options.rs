/*
Copyright (c) 2019, Guy Black
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

The views and conclusions contained in the software and documentation are those
of the authors and should not be interpreted as representing official policies,
either expressed or implied, of the FantASM project.
*/
extern crate argparse;
//extern crate chrono;

use std::path::Path;
use std::process::exit;

use argparse::{ArgumentParser, Store, StoreTrue};

use self::argparse::List;

#[derive(Default, Debug)]
pub struct Options {
    pub source: String,
    pub output: String,
    pub z80n: bool,
    pub verbose: bool,
    pub debug: bool,
    pub no_logo: bool,
    pub c_spect: bool,
    pub version: bool,
    pub include_dirs: Vec<String>,
    pub export_labels: String,
    pub origin: u16,
    pub max_code_size: isize,
    pub defines: Vec<String>,
}

impl Options {
    pub fn parse() -> Result<Options, String> {
        let description = format!("\nFantASM {} (Guy Fawkes) [{}]\n\u{000A9}2019 Captain Black\n", version!(), env!("BUILD_DATE"));
        let mut options = Options::default();
        {
            let mut parser = ArgumentParser::new();
            parser.set_description(&description);

            parser.refer(&mut options.source)
                .add_argument("source", Store, "Source file");

            parser.refer(&mut options.output)
                .add_argument("file", Store, "Output file");

            parser.refer(&mut options.z80n)
                .add_option(&["-N", "--z80n"], StoreTrue, "Enable Z80n (ZX Next) cpu extensions");

            parser.refer(&mut options.c_spect)
                .add_option(&["-c", "--cspect"], StoreTrue, "Enable cspect \"exit\" and \"break\" instructions");

            parser.refer(&mut options.no_logo)
                .add_option(&["-n", "--nologo"], StoreTrue, "Do no display the program name and version");

            parser.refer(&mut options.verbose)
                .add_option(&["-v", "--verbose"], StoreTrue, "Enable verbose output");

            parser.refer(&mut options.version)
                .add_option(&["-V", "--version"], StoreTrue, "Displays the version and exits");

            parser.refer(&mut options.include_dirs)
                .metavar("file")
                .add_option(&["-I", "--include"], List, "Add a directory to search for include files");

            parser.refer(&mut options.defines)
                .metavar("constant")
                .add_option(&["-D", "--define"], List, "Define 1 more constants");

            parser.refer(&mut options.export_labels)
                .metavar("file")
                .add_option(&["-e", "--export-labels"], Store, "Export labels to a file");

            parser.refer(&mut options.origin)
                .metavar("address")
                .add_option(&["-O", "--origin"], Store, "Address to start assembling code");

            parser.refer(&mut options.max_code_size)
                .metavar("size")
                .add_option(&["-M", "--max-code-size"], Store, "Limit the size of assembled code");

            parser.parse_args_or_exit();
        }

        if options.version {
            white_ln!("FantASM {}", version!());
            exit(0);
        }

        if !options.no_logo {
            white_ln!("{}",description);
        }

        if options.source.is_empty() {
            return Err(String::from("<source> is required"));
        }

        if options.output.is_empty() {
            return Err(String::from("<output> is required"));
        }

        if !Path::new(&options.source).exists() {
            red_ln!("Source file: {}-does not exist", options.source);
            exit(1);
        }

        Ok(options)
    }
}