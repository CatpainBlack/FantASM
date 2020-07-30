use std::cmp::max;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use pad::PadStr;

use crate::assembler::assembler_context::AssemblerContext;
use crate::assembler::error::Error;
use crate::assembler::error_type::ErrorType;

pub trait Label {
    fn add_label(&mut self, name: String, global: bool) -> Result<(), Error>;
    fn get_label(&mut self, name: &str) -> Option<isize>;
    fn is_label_defined(&self, name: &str) -> bool;
    fn export_labels(&mut self, file_name: &str) -> Result<(), Error>;
    fn mark_label_used(&mut self, name: &str);
}

impl Label for AssemblerContext {
    fn add_label(&mut self, name: String, global: bool) -> Result<(), Error> {
        let mut label_name = name.clone();

        if label_name.ends_with(":") {
            label_name = name.replace(":", "");
        }
        if !label_name.starts_with(".") {
            self.label_context = label_name.clone();
        } else {
            label_name = self.label_context.clone() + &label_name.clone();
        }

        if self.case_insensitive {
            label_name = label_name.to_uppercase()
        }

        if self.is_label_defined(label_name.as_str()) {
            return Err(self.error(ErrorType::LabelOrConstantExists));
        }
        self.labels.insert(label_name.to_string(), self.current_pc);
        if global {
            self.global_labels.push(label_name.to_string());
        }
        self.used.insert(label_name, false);
        Ok(())
    }

    fn get_label(&mut self, name: &str) -> Option<isize> {
        let mut label_name = name.to_string();

        if label_name.starts_with(".") {
            label_name = self.label_context.clone() + &label_name.clone();
        }

        if self.case_insensitive {
            label_name = label_name.to_uppercase()
        }
        match self.labels.get(&label_name).cloned() {
            None => None,
            Some(s) => {
                self.mark_label_used(&label_name);
                Some(s)
            }
        }
    }
    fn is_label_defined(&self, name: &str) -> bool {
        let mut label_name = name.to_string();

        if label_name.starts_with(".") {
            label_name = self.label_context.clone() + &label_name.clone();
        }
        if self.case_insensitive {
            label_name = label_name.to_uppercase()
        }
        self.labels.contains_key(&label_name)
    }

    fn export_labels(&mut self, file_name: &str) -> Result<(), Error> {
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

    fn mark_label_used(&mut self, name: &str) {
        if self.used.contains_key(name) {
            *self.used.get_mut(name).unwrap() = true;
        }
    }
}
