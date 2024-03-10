use std::fmt::{Display, Formatter};
use crate::datarow::DataRowError;

//Simple structure for storing a single warning/error and displaying it.
#[derive(Debug)]
pub struct LoadWarning {
    line_index: usize,
    message: DataRowError
}

impl LoadWarning {
    pub fn new(row_num: usize, warning: DataRowError) -> LoadWarning {
        LoadWarning {
            line_index: row_num,
            message: warning
        }
    }
}

impl Display for LoadWarning {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Row {} {:?}", self.line_index, self.message)
    }
}