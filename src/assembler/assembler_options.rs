use std::string::ToString;

use crate::assembler::Assembler;

pub trait AssemblerOptions {
    fn enable_z80n(&mut self, enabled: bool) -> &mut Assembler;
    fn enable_console(&mut self, enabled: bool) -> &mut Assembler;
    fn enable_cspect(&mut self, enabled: bool) -> &mut Assembler;
    fn enable_debug(&mut self, enabled: bool) -> &mut Assembler;
    fn add_include_dirs(&mut self, dirs: Vec<String>) -> &mut Assembler;
    fn add_defines(&mut self, defines: Vec<String>) -> &mut Assembler;
    fn export_labels(&mut self, file_name: &str) -> &mut Assembler;
    fn origin(&mut self, address: u16) -> &mut Assembler;
    fn max_code_size(&mut self, size: usize) -> &mut Assembler;
}

impl AssemblerOptions for Assembler {
    fn enable_z80n(&mut self, enabled: bool) -> &mut Assembler {
        self.z80n_enabled = enabled;
        self
    }

    fn enable_console(&mut self, enabled: bool) -> &mut Assembler {
        self.console_output = enabled;
        self
    }

    fn enable_cspect(&mut self, enabled: bool) -> &mut Assembler {
        self.c_spect_enabled = enabled;
        self
    }

    fn enable_debug(&mut self, enabled: bool) -> &mut Assembler {
        self.debug = enabled;
        self
    }

    fn add_include_dirs(&mut self, dirs: Vec<String>) -> &mut Assembler {
        self.include_dirs = dirs.clone();
        self
    }

    fn add_defines(&mut self, defines: Vec<String>) -> &mut Assembler {
        self.defines = defines.clone();
        self
    }

    fn export_labels(&mut self, file_name: &str) -> &mut Assembler {
        self.labels_file = file_name.to_string();
        self
    }

    fn origin(&mut self, address: u16) -> &mut Assembler {
        self.origin = address as isize;
        self.context.pc(self.origin);
        self
    }

    fn max_code_size(&mut self, size: usize) -> &mut Assembler {
        if size > 0 {
            self.bank.max_code_size(size);
        } else {
            self.bank.max_code_size(65536);
        }
        self
    }
}