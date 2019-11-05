use std::collections::HashMap;

use asciimath::{eval, scope};

use crate::assembler::error::{Error, ErrorLevel};
use crate::assembler::error_type::ErrorType;
use crate::assembler::ForwardReference;
use crate::assembler::label::Label;

#[derive(Default)]
pub struct AssemblerContext {
    pub(super)labels: HashMap<String, isize>,
    pub(super)global_labels: Vec<String>,
    pub(super)constants: HashMap<String, isize>,
    pub(super)size_of: HashMap<String, isize>,
    pub(super)struct_defs: HashMap<String, HashMap<String, isize>>,
    pub(super)forward_references: Vec<ForwardReference>,
    pub(super)line_number: Vec<isize>,
    pub(super)file_name: Vec<String>,
    pub(super)current_pc: isize,
    pub(super)label_context: String,
    pub(super)asm_pc: isize,
    pub(super)next_label_global: bool,
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

    pub fn enter(&mut self, name: &str, defines: &Vec<String>) {
        for s in defines {
            if s.contains("=") {
                let t: Vec<&str> = s.split("=").collect();
                let label = t[0];
                let value = match eval(t[1], &scope! {}) {
                    Ok(n) => n as isize,
                    Err(e) => {
                        println!("Error: {}", e);
                        0
                    }
                };
                self.constants.insert(label.to_string(), value);
            } else {
                println!("Invalid Define {}", s);
            }
        }
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

    pub fn get_label_or_constant_value(&mut self, name: &str) -> Result<isize, Error> {
        if let Some(address) = self.get_label(name) {
            return Ok(address);
        }
        if let Some(&address) = self.constants.get(name) {
            return Ok(address);
        }
        Err(self.error(ErrorType::LabelNotFound))
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


    pub fn next_forward_ref(&mut self) -> Option<ForwardReference> {
        self.forward_references.pop()
    }

    pub fn add_forward_ref(&mut self, fw: ForwardReference) {
        self.forward_references.push(fw);
    }
}