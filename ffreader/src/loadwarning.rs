use std::fmt::{Display, Formatter};

/// Simple structure for storing a single warning/error and displaying it.
pub struct LoadWarning {
    line_index: usize,
    message: Box<dyn Display>
}

impl LoadWarning {
    /// Instantiates a new LoadWarning
    pub fn new(line_index: usize, message: Box<dyn Display>) -> LoadWarning {
        LoadWarning {
            line_index,
            message
        }
    }
}

impl Display for LoadWarning {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {} {}", self.line_index + 1, self.message)
    }
}