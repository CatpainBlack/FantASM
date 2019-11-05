use crate::assembler::assembler_context::AssemblerContext;
use crate::assembler::error::Error;
use crate::assembler::error_type::ErrorType;

pub trait Constant {
    fn is_constant_defined(&self, name: &str) -> bool;
    fn get_constant(&mut self, name: &str) -> Option<isize>;
    fn add_constant(&mut self, name: String, value: isize) -> Result<(), Error>;
}

impl Constant for AssemblerContext {
    fn is_constant_defined(&self, name: &str) -> bool {
        self.constants.contains_key(name)
    }

    fn get_constant(&mut self, name: &str) -> Option<isize> {
        self.constants.get(name).cloned()
    }

    fn add_constant(&mut self, name: String, value: isize) -> Result<(), Error> {
        if self.is_constant_defined(name.as_str()) {
            return Err(self.error(ErrorType::LabelOrConstantExists));
        }
        self.constants.insert(name, value);
        Ok(())
    }
}