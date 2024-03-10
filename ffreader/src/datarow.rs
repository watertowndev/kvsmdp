use std::fmt::{Display, Formatter};
use crate::{DataField, DataFieldDef, DataFieldError};

/// Holds a list of the fields found in a row.
#[derive(Debug)]
pub struct DataRow {
    fields: Vec<DataField>
}

pub enum DataRowError {
    FieldError(DataFieldError),
    BadRowLength(usize),
    FieldNameNotFound(String)
}

pub type Result<T> = std::result::Result<T, DataRowError>;

impl Display for DataRowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DataRowError::FieldError(fe) => fe.to_string(),
            DataRowError::BadRowLength(l) => format!("Bad Row Length ({})", l),
            DataRowError::FieldNameNotFound(n) => format!("Field Name Not Found ({})", n)
        };
        write!(f, "{}", s)
    }
}

impl From<DataFieldError> for DataRowError {
    fn from(value: DataFieldError) -> Self {
        DataRowError::FieldError(value)
    }
}

impl DataRow {
    const MINIMUM_LENGTH: usize = 183;

    /// Try to create a DataRow structure using the definitions provided.
    pub fn try_create(row: &str, row_defs: &Vec<DataFieldDef>) -> Result<DataRow> {
        if row.len() < Self::MINIMUM_LENGTH {
            return Err(DataRowError::BadRowLength(row.len()))
        }

        let tfs = DataField::try_from_row;
        let mut fields = Vec::new();

        for row_def in row_defs {
            fields.push(tfs(row, row_def)?);
        }

        Ok(DataRow {
            fields
        })
    }

    /// Get a copy of the row with specified fields in order
    pub fn get_ordered_fields(&self, field_list: &Vec<&str>) -> Result<Vec<DataField>>{
        let mut list = vec![];

        for f in field_list {
            if let Some(c) = self.fields.iter().find(|n| n.name() == f) {
                list.push((*c).clone());
            }
            else {
                return Err(DataRowError::FieldNameNotFound(f.to_string()));
            }
        }

        Ok(list)
    }

    /// Get a reference to the DataFields contained in the struct.
    pub fn fields(&self) -> &Vec<DataField> {
        &self.fields
    }
}