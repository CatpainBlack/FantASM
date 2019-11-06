use crate::assembler::assembler_context::AssemblerContext;
use crate::assembler::label::Label;

pub trait SizeOfHandler {
    fn get_size_of(&mut self, label: &str) -> Option<isize>;
    fn add_size_of_struct(&mut self, name: &str, size: isize);
    fn add_size_of(&mut self, size: isize);
}

impl SizeOfHandler for AssemblerContext {
    fn get_size_of(&mut self, label: &str) -> Option<isize> {
        if !self.size_of.contains_key(label) {
            None
        } else {
            Some(self.size_of[label])
        }
    }

    fn add_size_of_struct(&mut self, name: &str, size: isize) {
        self.size_of.insert(name.to_string(), size);
    }

    fn add_size_of(&mut self, size: isize) {
        let label = self.label_context.to_string();
        if let Some(pc) = self.get_label(&label) {
            if self.current_pc == pc {
                self.size_of.insert(self.label_context.to_string(), size);
            }
        }
    }
}