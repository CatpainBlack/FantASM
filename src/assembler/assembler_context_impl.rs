use crate::assembler::{AssemblerContext, Error, ErrorLevel};
use crate::assembler::error_impl::ErrorType;


impl AssemblerContext {
    pub fn current_line_number(&self) -> isize {
        *self.line_number.last().unwrap_or(&0isize)
    }

    pub fn current_file_name(&self) -> String {
        self.file_name.last().unwrap_or(&"".to_string()).to_string()
    }

    pub fn is_label_defined(&self, name: &str) -> bool {
        self.labels.contains_key(name)
    }

    pub fn get_label_or_constant_value(&mut self, name: &str) -> Result<isize, Error> {
        if let Some(&address) = self.labels.get(name) {
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

    pub fn add_label(&mut self, name: String) -> Result<(), Error> {
        if self.is_label_defined(name.as_str()) {
            return Err(self.error(ErrorType::LabelOrConstantExists));
        }
        self.labels.insert(name, self.current_pc);
        Ok(())
    }

    pub fn add_constant(&mut self, name: String, value: isize) -> Result<(), Error> {
        if self.constants.contains_key(name.as_str()) {
            return Err(self.error(ErrorType::LabelOrConstantExists));
        }
        self.constants.insert(name, value);
        Ok(())
    }
}