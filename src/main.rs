#[macro_use]
extern crate colour;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate version;

use std::process::exit;
use std::time::Instant;

use crate::assembler::Assembler;
use crate::assembler::assembler_options::AssemblerOptions;
use crate::assembler::error::Error;
use crate::options::Options;

mod options;
mod assembler;

fn main() {
    match _main() {
        Ok(_) => exit(0),
        Err(e) => {
            red_ln!("Error - {}", e.message);
            exit(1);
        }
    }
}

fn _main() -> Result<(), Error> {
    let options = Options::parse()?;

    let mut assembler = Assembler::new();

    assembler
        .enable_cspect(options.c_spect)
        .enable_z80n(options.z80n)
        .enable_console(options.verbose)
        .enable_debug(options.debug)
        .add_include_dirs(options.include_dirs)
        .add_defines(options.defines)
        .export_labels(&options.export_labels)
        .origin(options.origin)
        .max_code_size(options.max_code_size as usize)
        .case_insensitive(options.case_insensitive_labels);

    let now = Instant::now();
    if options.verbose {
        println!("Assembling: {}",options.source);
    }


    match assembler.assemble(options.source.as_str()) {
        Ok(_) => assembler.save_raw(&options.output)?,
        Err(e) => {
            red_ln!("[{} : {}] {}",e.file_name,e.line_no,e.message);
            exit(1);
        }
    }

    if options.warnings {
        assembler.display_unused();
    }

    if options.verbose {
        println!("Assembly complete [{}s]", (now.elapsed().as_millis() as f64)/1000f64);
    }

    Ok(())
}
