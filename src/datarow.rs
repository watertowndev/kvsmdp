/*
Original specification from the previous version of this utility:

    ("AccountNo1", {"width": 10, "padding": 1, "handler": stripPadding}),  # ACCOUNT NO
    ("CyclNo1", {"width": 2, "padding": 3, "handler": stripPadding}),  # CYCL NO
    ("Status", {"width": 1, "padding": 6, "handler": stripPadding}),  # STATUS
    ("OwnerName", {"width": 25, "padding": 1, "handler": stripPadding}),  # LOCATION PERSON
    ("PropAddrKey", {"width": 10, "padding": 1, "handler": stripPadding}),  # PROP ADDR 1
    ("AddrLine2", {"width": 25, "padding": 1, "handler": stripPadding}),  # PROP ADDR 2
    ("MeterID", {"width": 13, "padding": 1, "handler": validateID}),  # ACCOUNT NO (this one has the meter no appended)
    ("CyclNo2", {"width": 2, "padding": 3, "handler": stripPadding}),  # CYCL NO
    ("ReadDigits", {"width": 1, "padding": 4, "handler": stripPadding}),  # M SIZE
    ("No", {"width": 1, "padding": 3, "handler": stripPadding}),  # NO
    ("Type", {"width": 1, "padding": 4, "handler": stripPadding}),  # TYPE
    ("ARB", {"width": 10, "padding": 1, "handler": stripPadding}),  # ARB
    ("FileKey", {"width": 10, "padding": 1, "handler": stripPadding}),  # FILE KEY
    ("StreetDirection", {"width": 1, "padding": 9, "handler": stripPadding}),  # DIRECTION
    ("StreetName", {"width": 25, "padding": 1, "handler": stripPadding}),  # STREET NAME
    ("StreetNumber", {"width": 6, "padding": 1, "handler": stripZeros}),  # NUMBER
    ("StreetUnit", {"width": 6, "padding": 1, "handler": stripPadding}),  # UNIT
    ("MeterSerial", {"width": 8, "padding": 6, "handler": stripPadding}),  # M NO
    ("PrintKey", {"width": 25, "padding": 1, "handler": fixPrintKey}),  # PRINT KEY
    ("MeterSize", {"width": 1, "padding": 5, "handler": decodeSizes}),  # PIPE SIZE
    ("Special", {"width": 1, "padding": 4, "handler": decodeSpecial})  # USER2 (special status flag
 */
use crate::datafield::{DataField, DataFieldDef, DataFieldError};

/// Holds a list of the fields found in a row.
#[derive(Debug)]
pub struct DataRow {
    fields: Vec<DataField>
}



#[derive(Debug)]
pub enum DataRowError {
    FieldError(DataFieldError),
    BadRowLength(usize),
    FieldNameNotFound(String)
}

impl From<DataFieldError> for DataRowError {
    fn from(value: DataFieldError) -> Self {
        DataRowError::FieldError(value)
    }
}

impl DataRow {
    const MINIMUM_LENGTH: usize = 183;

    /// Try to create a DataRow structure using the definitions provided.
    pub fn try_create(row: &str, row_defs: &Vec<DataFieldDef>) -> Result<DataRow, DataRowError> {
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
    pub fn get_ordered_fields(&self, field_list: &Vec<&str>) -> Result<Vec<DataField>, DataRowError>{
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