#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate colour;
#[macro_use]
extern crate version;

use std::time::Instant;

use crate::assembler::{Assembler, Error};
use crate::options::Options;

mod options;
mod assembler;

fn main() -> Result<(), Error> {
    let options = Options::parse()?;

    if !options.no_logo {
        white_ln!("FantASM {} - (C)2019 Captain Black",version!());
    }


    let mut assembler = Assembler::new();
    assembler
        .enable_cspect(options.c_spect)
        .enable_z80n(options.z80n)
        .enable_console(options.verbose);

    let now = Instant::now();
    match assembler.assemble(options.source.as_str()) {
        Ok(_) => assembler.save_raw(&options.output)?,
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
