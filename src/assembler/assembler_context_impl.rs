use crate::assembler::{Error, ErrorLevel, ForwardReference};
use crate::assembler::error_impl::ErrorType;
use std::collections::HashMap;

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

    pub fn current_pc(&mut self) -> isize {
        self.current_pc
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
}