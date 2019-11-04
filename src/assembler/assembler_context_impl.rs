use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use asciimath::{eval, scope};
use pad::PadStr;

use crate::assembler::{Error, ErrorLevel, ForwardReference};
use crate::assembler::error_impl::ErrorType;

#[derive(Default)]
pub struct AssemblerContext {
    labels: HashMap<String, isize>,
    global_labels: Vec<String>,
    constants: HashMap<String, isize>,
    size_of: HashMap<String, isize>,
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

    pub fn add_label(&mut self, name: String, global: bool) -> Result<(), Error> {
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
        self.labels.insert(label_name.to_string(), self.current_pc);
        if global {
            self.global_labels.push(label_name.to_string());
        }
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
            for g in self.global_labels.clone() {
                let s = self.get_label(&g.clone()).unwrap_or(-1);
                let line = format!("{} = 0x{:x}\n", g.pad_to_width(m), s);
                file.write(line.as_bytes())?;
            }
        }
        Ok(())
    }

    pub fn get_size_of(&mut self, label: &str) -> Option<isize> {
        if !self.size_of.contains_key(label) {
            None
        } else {
            Some(self.size_of[label])
        }
    }

    pub fn add_size_of_struct(&mut self, name: &str, size: isize) {
        self.size_of.insert(name.to_string(), size);
    }

    pub fn add_size_of(&mut self, size: isize) {
        let label = self.label_context.to_string();
        if let Some(pc) = self.get_label(&label) {
            if self.current_pc == pc {
                self.size_of.insert(self.label_context.to_string(), size);
                //println!("Added sizeof({},{})", self.label_context, size);
            }
        }
    }
}