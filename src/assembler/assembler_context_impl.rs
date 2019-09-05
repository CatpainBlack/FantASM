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

#[derive(Default)]
pub struct AssemblerContext {
    labels: HashMap<String, isize>,
    constants: HashMap<String, isize>,
    forward_references: Vec<ForwardReference>,
    line_number: Vec<isize>,
    file_name: Vec<String>,
    current_pc: isize,
    label_context: String,
}

impl AssemblerContext {
    pub fn current_line_number(&self) -> isize {
        *self.line_number.last().unwrap_or(&0isize)
    }

    pub fn current_file_name(&self) -> String {
        self.file_name.last().unwrap_or(&"<none>".to_string()).to_string()
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
        self.labels.contains_key(name)
    }
    pub fn is_constant_defined(&self, name: &str) -> bool { self.constants.contains_key(name) }

    pub fn get_label_or_constant_value(&mut self, name: &str) -> Result<isize, Error> {
        if let Some(&address) = self.labels.get(name) {
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
        self.labels.get(name).cloned()
    }

    pub fn error(&mut self, t: ErrorType) -> Error {
        Error {
            line_no: self.current_line_number(),
            message: t.to_string(),
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

    pub fn try_resolve_label(&mut self, name: &str, pc_offset: isize, relative: bool) -> u16 {
        let mut addr = 0u16;
        let mut label_name = name.to_string();
        if label_name.starts_with(".") {
            label_name = self.label_context.clone() + &label_name.clone();
        }

        if let Some(a) = self.constants.get(&label_name) {
            addr = *a as u16;
        } else if let Some(a) = self.labels.get(&label_name) {
            addr = *a as u16;
        } else {
            self.forward_references.push(ForwardReference {
                is_expression: false,
                pc: self.current_pc + pc_offset,
                label: label_name.to_string(),
                expression: vec![],
                is_relative: relative,
                byte_count: 2,
                line_no: self.current_line_number(),
                file_name: self.current_file_name(),
            });
        }
        return addr;
    }

    pub fn next_forward_ref(&mut self) -> Option<ForwardReference> {
        self.forward_references.pop()
    }

    pub fn add_forward_ref(&mut self, fw: ForwardReference) {
        self.forward_references.push(fw);
    }

    pub fn dump(&mut self) {
        magenta_ln!("Labels            : {:?}", self.labels);
        magenta_ln!("Constants         : {:?}", self.constants);
        magenta_ln!("Forward References: {:?}", self.forward_references);
    }
}