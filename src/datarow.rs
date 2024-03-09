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
use crate::datafield;
use crate::datafield::{DataField, DataFieldError};

#[derive(Debug)]
pub struct DataRow {
    row: Vec<DataField>
}

#[derive(Debug)]
pub enum DataRowError {
    FieldError(DataFieldError),
    BadRowLength(usize)
}

impl From<DataFieldError> for DataRowError {
    fn from(value: DataFieldError) -> Self {
        DataRowError::FieldError(value)
    }
}

impl DataRow {
    const MINIMUM_LENGTH: usize = 183;

    pub fn try_create(row: &str) -> Result<DataRow, DataRowError> {
        if row.len() < Self::MINIMUM_LENGTH {
            return Err(DataRowError::BadRowLength(row.len()))
        }

        let tfr = DataField::try_from_row;
        let mut fields = Vec::new();

        fields.push(tfr(row, "AccountNo1", 0, 11, &datafield::validate_acct)?);
        fields.push(tfr(row, "CyclNo1", 11, 16, &datafield::cleanup)?);
        fields.push(tfr(row, "Status", 16, 23, &datafield::cleanup)?);
        fields.push(tfr(row, "OwnerName", 23, 49, &datafield::cleanup)?);
        fields.push(tfr(row, "PropAddrKey", 49, 60, &datafield::cleanup)?);
        fields.push(tfr(row, "AddrLine2", 60, 86, &datafield::cleanup)?);
        fields.push(tfr(row, "MeterID", 86, 100, &datafield::validate_acct)?);
        fields.push(tfr(row, "CyclNo2", 100, 105, &datafield::cleanup)?);
        fields.push(tfr(row, "ReadDigits", 105, 110, &datafield::cleanup)?);
        fields.push(tfr(row, "No", 110, 114, &datafield::cleanup)?);
        fields.push(tfr(row, "Type", 114, 119, &datafield::cleanup)?);
        fields.push(tfr(row, "ARB", 119, 130, &datafield::cleanup)?);
        fields.push(tfr(row, "FileKey", 130, 141, &datafield::cleanup)?);
        fields.push(tfr(row, "StreetDirection", 141, 151, &datafield::cleanup)?);
        fields.push(tfr(row, "StreetName", 151, 177, &datafield::cleanup)?);
        fields.push(tfr(row, "StreetNumber", 177, 184, &datafield::trim_zeroes)?);
        fields.push(tfr(row, "StreetUnit", 184, 191, &datafield::cleanup)?);
        fields.push(tfr(row, "MeterSerial", 191, 205, &datafield::cleanup)?);
        fields.push(tfr(row, "PrintKey", 205, 231, &datafield::fix_printkey)?);
        fields.push(tfr(row, "MeterSize", 231, 237, &datafield::fix_meter_size)?);
        fields.push(tfr(row, "Special", 237, 242, &datafield::decode_special)?);
        //well that was tedious (╯°□°）╯︵ ┻━┻

        Ok(DataRow {
            row: fields
        })
    }
}