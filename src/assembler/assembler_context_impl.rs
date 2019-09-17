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

use std::collections::HashMap;

use crate::assembler::{Error, ErrorLevel, ForwardReference};
use crate::assembler::error_impl::ErrorType;
use std::cmp::max;
use pad::PadStr;
use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Default)]
pub struct AssemblerContext {
    labels: HashMap<String, isize>,
    constants: HashMap<String, isize>,
    forward_references: Vec<ForwardReference>,
    line_number: Vec<isize>,
    file_name: Vec<String>,
    current_pc: isize,
    pub(crate)label_context: String,
    asm_pc: isize,
}

impl AssemblerContext {
    pub fn current_line_number(&self) -> isize {
        *self.line_number.last().unwrap_or(&0isize)
    }

    pub fn current_file_name(&self) -> String {
        self.file_name.last().unwrap_or(&"<none>".to_string()).to_string()
    }

    pub fn is_included(&self, name: &String) -> bool {
        self.file_name.contains(name)
    }

    pub fn offset_pc(&mut self, offset: isize) -> isize {
        self.current_pc + offset
    }

    pub fn pc(&mut self, value: isize) {
        self.current_pc = value;
    }

    pub fn pc_add(&mut self, value: isize) {
        self.current_pc += value;
    }

    pub fn asm_pc(&self) -> isize { self.asm_pc }
    pub fn init_asm_pc(&mut self) { self.asm_pc = self.current_pc; }

    pub fn result<T>(&mut self, r: Result<T, ErrorType>) -> Result<T, Error> {
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.error(e)),
        }
    }

    pub fn enter(&mut self, name: &str) {
        self.file_name.push(name.to_string());
        self.line_number.push(0);
    }

    pub fn leave(&mut self) {
        self.file_name.pop();
        self.line_number.pop();
    }

    pub fn next_line(&mut self) {
        let len = self.line_number.len() - 1;
        self.line_number[len] += 1;
    }

    pub fn is_label_defined(&self, name: &str) -> bool {
        let mut label_name = name.to_string();
        if label_name.starts_with(".") {
            label_name = self.label_context.clone() + &label_name.clone();
        }
        self.labels.contains_key(&label_name)
    }
    pub fn is_constant_defined(&self, name: &str) -> bool {
        self.constants.contains_key(name)
    }

    pub fn get_label_or_constant_value(&mut self, name: &str) -> Result<isize, Error> {
        if let Some(address) = self.get_label(name) {
            return Ok(address);
        }
        if let Some(&address) = self.constants.get(name) {
            return Ok(address);
        }
        Err(self.error(ErrorType::LabelNotFound))
    }

    pub fn get_constant(&mut self, name: &str) -> Option<isize> {
        self.constants.get(name).cloned()
    }

    pub fn get_label(&mut self, name: &str) -> Option<isize> {
        let mut label_name = name.to_string();
        if label_name.starts_with(".") {
            label_name = self.label_context.clone() + &label_name.clone();
        }
        self.labels.get(&label_name).cloned()
    }

    pub fn error(&mut self, t: ErrorType) -> Error {
        Error {
            line_no: self.current_line_number(),
            message: t.to_string(),
            level: ErrorLevel::Fatal,
            file_name: self.current_file_name(),
        }
    }

    pub fn error_text(&mut self, t: ErrorType, text: &str) -> Error {
        let message = format!("{} - {}", t.to_string(), text);
        Error {
            line_no: self.current_line_number(),
            message,
            level: ErrorLevel::Fatal,
            file_name: self.current_file_name(),
        }
    }

    pub fn add_label(&mut self, name: String) -> Result<(), Error> {
        let mut label_name = name.clone();
        if label_name.ends_with(":") {
            label_name = name.replace(":", "");
        }
        if !label_name.starts_with(".") {
            self.label_context = label_name.clone();
        } else {
            label_name = self.label_context.clone() + &label_name.clone();
        }

        if self.is_label_defined(label_name.as_str()) {
            return Err(self.error(ErrorType::LabelOrConstantExists));
        }
        self.labels.insert(label_name, self.current_pc);
        Ok(())
    }

    pub fn add_constant(&mut self, name: String, value: isize) -> Result<(), Error> {
        if self.is_constant_defined(name.as_str()) {
            return Err(self.error(ErrorType::LabelOrConstantExists));
        }
        self.constants.insert(name, value);
        Ok(())
    }

    pub fn next_forward_ref(&mut self) -> Option<ForwardReference> {
        self.forward_references.pop()
    }

    pub fn add_forward_ref(&mut self, fw: ForwardReference) {
        self.forward_references.push(fw);
    }

    pub fn export_labels(&mut self, file_name: &str) -> Result<(), Error> {
        if file_name.len() > 0 {
            let path = Path::new(file_name);
            let mut file = BufWriter::new(File::create(&path)?);

            let mut m = 0;
            for (l, _) in &self.labels {
                m = max(m, l.len() + 1);
            }
            for (l, s) in &self.labels {
                let line = format!("{} = 0x{:x}\n", l.pad_to_width(m), s);
                file.write(line.as_bytes())?;
            }
        }
        Ok(())
    }

//    pub fn dump(&mut self) {
//        magenta_ln!("Labels            : {:?}", self.labels);
//        magenta_ln!("Constants         : {:?}", self.constants);
//        magenta_ln!("Forward References: {:?}", self.forward_references);
//    }
}