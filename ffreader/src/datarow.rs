use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::{DataField, DataFieldDef, DataFieldError};

/// Holds a list of the fields found in a row.
#[derive(Debug)]
pub struct DataRow {
    fields: Vec<DataField>
}

/// Errors that DataRows may encounter.
pub enum DataRowError {
    /// A field-specific error (contains details).
    FieldError(DataFieldError),
    /// A row length is out of bounds.
    BadRowLength(usize),
    /// A field name was specified but not found.
    FieldNameNotFound(String)
}

/// Convenient Result shorthand for DataRowError results.
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

impl Debug for DataRowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error for DataRowError {}

impl From<DataFieldError> for DataRowError {
    fn from(value: DataFieldError) -> Self {
        DataRowError::FieldError(value)
    }
}

impl DataRow {
    const MINIMUM_LENGTH: usize = 183; // todo: make this configurable

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

#[cfg(test)]
mod tests {
    use crate::DataFieldResult;
    use super::*;

    fn test_row() -> String {
        String::from("5412345678  54     1   123 TEST OWNER            5412345678                           5412345678001  54    4    1  Z              5412345678           TEST AVE ABC              000075                      0-0001-111.000            0        R")
    }

    fn echo_ok(s: String) -> DataFieldResult<String> { Ok(s) }

    fn test_field_defs() -> Vec<DataFieldDef<'static>> {
        vec![
            DataFieldDef::new("AccountNo1", 0, 11, &echo_ok),
            DataFieldDef::new("CyclNo1", 11, 16, &echo_ok),
            DataFieldDef::new("Status", 16, 23, &echo_ok),
            DataFieldDef::new("OwnerName", 23, 49, &echo_ok),
            DataFieldDef::new("PropAddrKey", 49, 60, &echo_ok),
            DataFieldDef::new("AddrLine2", 60, 86, &echo_ok),
            DataFieldDef::new("MeterID", 86, 100, &echo_ok),
            DataFieldDef::new("CyclNo2", 100, 105, &echo_ok),
            DataFieldDef::new("ReadDigits", 105, 110, &echo_ok),
            DataFieldDef::new("No", 110, 114, &echo_ok),
            DataFieldDef::new("Type", 114, 119, &echo_ok),
            DataFieldDef::new("ARB", 119, 130, &echo_ok),
            DataFieldDef::new("FileKey", 130, 141, &echo_ok),
            DataFieldDef::new("StreetDirection", 141, 151, &echo_ok),
            DataFieldDef::new("StreetName", 151, 177, &echo_ok),
            DataFieldDef::new("StreetNumber", 177, 184, &echo_ok),
            DataFieldDef::new("StreetUnit", 184, 191, &echo_ok),
            DataFieldDef::new("MeterSerial", 191, 205, &echo_ok),
            DataFieldDef::new("PrintKey", 205, 231, &echo_ok),
            DataFieldDef::new("MeterSize", 231, 237, &echo_ok),
            DataFieldDef::new("Special", 237, 242, &echo_ok)
        ]
    }

    #[test]
    fn creation_extraction_works() {
        let row = test_row();
        let defs = test_field_defs();

        let datarow = DataRow::try_create(&row, &defs).unwrap();

        let fields = datarow.fields();

        assert_eq!(fields.iter().find(|s| s.name() == "MeterID").unwrap().data(), "5412345678001");
        assert_eq!(fields.iter().find(|s| s.name() == "AccountNo1").unwrap().data(), "5412345678");
        assert_eq!(fields.iter().find(|s| s.name() == "Special").unwrap().data(), "R");
        assert_eq!(fields.iter().find(|s| s.name() == "MeterSize").unwrap().data(), "0");
        assert_eq!(fields.iter().find(|s| s.name() == "OwnerName").unwrap().data(), "123 TEST OWNER");
    }
}