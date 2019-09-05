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

#[macro_use]
extern crate colour;
#[macro_use]
extern crate lazy_static;
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
