#[macro_use]
extern crate colour;

use std::time::Instant;

use crate::assembler::{Assembler, Error};
use crate::options::Options;

mod options;
mod assembler;

fn main() -> Result<(), Error> {
    let options = Options::parse()?;

    if !options.nologo {
        white_ln!("FantASM 0.7.1 - (C)2019 Captain Black");
    }


    let mut assembler = Assembler::new();
    if options.verbose {
        assembler.enable_console();
    }

    if options.z80n {
        assembler.enable_z80n();
    }

    let now = Instant::now();
    match assembler.assemble(options.source.as_str()) {
        Ok(_) => assembler.save_raw(options.output.as_str())?,
        Err(e) => {
            red_ln!("[{} : {}] {}",e.file_name,e.line_no,e.message);
        }
    }

    if options.verbose {
        dark_yellow_ln!("Assembly complete [{}s]", (now.elapsed().as_millis() as f64)/1000f64);
    }

    if options.debug {
        assembler.dump();
    }

    Ok(())
}
