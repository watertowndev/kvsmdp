
/// Contains a datafield, including name, raw data, and processes data (if any).
#[derive(Debug, Clone)]
pub struct DataField {
    name: String,
    raw: String,
    data: Option<String>
}

/// Errors that DataFields may encounter.
#[derive(Debug)]
pub enum DataFieldError {
    StartAfterEnd(String),
    NonASCIIRow(String),
    InvalidOrExcludedAccountID(String),
    InvalidMeterSize(String),
    InvalidSpecialCode(String),
    BadNumber(String),
    FieldContainsQuote
}

type Result<T> = std::result::Result<T, DataFieldError>;

/// Holds details pertaining to the structure of a row and the desired post-processing function.
pub struct DataFieldDef<'a> {
    pub(crate) name: String,
    pub(crate) start_idx: usize,
    pub(crate) end_idx: usize,
    pub(crate) post_process: &'a dyn Fn(String) -> Result<String>
}

impl DataFieldDef<'_> {
    pub fn new(name: impl ToString, start_idx: usize, end_idx: usize,
               post_process: &dyn Fn(String)-> Result<String>) -> DataFieldDef {
        DataFieldDef {
            name: name.to_string(),
            start_idx,
            end_idx,
            post_process
        }
    }
}

impl DataField {
    pub fn new(name: &str, data: String) -> DataField {
        DataField {
            name: name.to_string(),
            raw: data.clone(),
            data: if data.len() == 0 {
                None
            } else {
                Some(data)
            },
        }
    }

    ///Try to create a DataField from a row using the provided data field definition.
    pub fn try_from_row(row: &str, field_def: &DataFieldDef) -> Result<DataField>
    {
        //fields can be optional and result in lines that are short
        //return nothing if the start is after the row (it's truncated)
        if field_def.start_idx > row.len() {
            return Ok(DataField {
                name: field_def.name.to_string(),
                raw: "".to_string(),
                data: None
            });
        }

        let end_idx = if field_def.end_idx > row.len() {
            row.len()
        }
        else {
            field_def.end_idx
        };

        if field_def.start_idx > end_idx {
            return Err(DataFieldError::StartAfterEnd(field_def.name.to_string()));
        }

        if !row.is_ascii() {
            return Err(DataFieldError::NonASCIIRow(field_def.name.to_string()));
        }

        let raw = row[field_def.start_idx..end_idx].to_string();
        let data = (field_def.post_process)(raw.trim().to_string())?;


        Ok(DataField {
            name: field_def.name.to_string(),
            raw,
            data: if data.len() == 0 {
                None
            } else {
                Some(data)
            },
        })
    }

    //Returns a reference to the name.
    pub fn name(&self) -> &String {
        &self.name
    }

    //returns a clone of the data. An empty string is returned if None.
    pub fn data(&self) -> String {
        self.data.clone().unwrap_or("".to_string())
    }
}

/// Check that the account starts with appropriate numbers
/// and that it isn't excluded intentionally.
/// Todo: this is datafile specific and should be extracted elsewhere.
pub fn validate_acct(value: String) -> Result<String> {
    let acct = cleanup(value)?;
    if acct.len() < 10 {
        Err(DataFieldError::InvalidOrExcludedAccountID(acct))
    }
    else if acct.starts_with("54777") {
        Err(DataFieldError::InvalidOrExcludedAccountID(acct))
    }
    else {
        match &acct[0..2] {
            "51" | "52" | "53" | "54" => Ok(acct),
            _ => Err(DataFieldError::InvalidOrExcludedAccountID(acct))
        }
    }
}

/// Remove whitespace from the beginning and end of value.
/// Converts ampersands to 'and' and commas to spaces.
/// Todo: this is datafile specific and should be extracted elsewhere.
pub fn cleanup(value: String) -> Result<String> {
    if value.contains("\"") {
        Err(DataFieldError::FieldContainsQuote)
    }
    else {
        Ok(value.trim().replace("&", "and").replace(",", " "))
    }
}

/// Normalize the PrintKey value.
/// Todo: this is datafile specific and should be extracted elsewhere.
pub fn fix_printkey(value: String) -> Result<String> {
    let printkey = cleanup(value)?;
    let mut npk: String;
    if printkey.len() == "0-0000-000.000".len() {
        npk = printkey[0..2].to_string();
        npk.push_str(&printkey[4..]);
        Ok(npk)
    }
    else if printkey.len() == "00-0000-000.000".len() {
        npk = printkey[0..3].to_string();
        npk.push_str(&printkey[5..]);
        Ok(npk)
    }
    else if printkey.len() == "000-0000-000.000".len() {
        npk = printkey[0..4].to_string();
        npk.push_str(&printkey[6..]);
        Ok(npk)
    }
    else {
        Ok(printkey)
    }
}

/// Convert the size reading or code into a normalized value.
/// Todo: this is datafile specific and should be extracted elsewhere.
pub fn fix_meter_size(value: String) -> Result<String> {
    let meter_size = cleanup(value)?;
    match meter_size.as_str() {
        "0" | "0.625" => Ok("0.625".to_string()),
        "5" | "0.75" => Ok("0.75".to_string()),
        "1" => Ok("1".to_string()),
        "7" | "1.5" => Ok("1.5".to_string()),
        "2" => Ok("2".to_string()),
        "3" => Ok("3".to_string()),
        "4" => Ok("4".to_string()),
        "6" => Ok("6".to_string()),
        "8" => Ok("8".to_string()),
        _ => Err(DataFieldError::InvalidMeterSize(meter_size))
    }
}

/// Decode the Special value into expanded terms.
/// Todo: this is datafile specific and should be extracted elsewhere.
pub fn decode_special(value: String) -> Result<String> {
    let special = cleanup(value)?;
    match special.as_str() {
        "S" => Ok("Shut".to_string()),
        "E" => Ok("Elderly Exemption".to_string()),
        "X" => Ok("Exempt".to_string()),
        "O" => Ok("Outside User".to_string()),
        "R" => Ok("Removed".to_string()),
        _ => Err(DataFieldError::InvalidSpecialCode(special))
    }
}

/// Trim leading zeroes. Will erase all zeroes if nothing else is present.
/// On failure or negative, returns an Ok(empty string).
/// Todo: this is datafile specific and should be extracted elsewhere.
pub fn trim_zeroes(value: String) -> Result<String> {
    let num = value.parse::<u32>();

    match num {
        Ok(x) => {
            if x <= 0 {
               Ok("".to_string())
            }
            else {
                Ok(x.to_string())
            }
        },
        Err(_) => Ok("".to_string())
    }
}
