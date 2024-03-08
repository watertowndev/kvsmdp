
#[derive(Debug)]
pub struct DataField {
    name: String,
    raw: String,
    data: Option<String>
}

#[derive(Debug)]
pub enum DataFieldError {
    StartAfterEnd(String),
    NonASCIIRow(String),
    InvalidOrExcludedAccountID(String),
    InvalidMeterSize(String),
    InvalidSpecialCode(String),
    BadNumber(String)
}

type Result<T> = std::result::Result<T, DataFieldError>;

impl DataField {
    pub fn try_from_row(row: &str, name: &str,
                        start_idx: usize, end_idx: usize, post_fn: &dyn Fn(String) -> Result<String>)
                        -> Result<DataField>
    {
        //fields can be optional and result in lines that are short
        if start_idx > row.len() || end_idx > row.len() {
            return Ok(DataField {
                name: name.to_string(),
                raw: "".to_string(),
                data: None
            });
        }

        if start_idx > end_idx {
            return Err(DataFieldError::StartAfterEnd(name.to_string()));
        }

        if !row.is_ascii() {
            return Err(DataFieldError::NonASCIIRow(name.to_string()));
        }

        let raw = row[start_idx..end_idx].to_string();
        let data = post_fn(raw.trim().to_string())?;


        Ok(DataField {
            name: name.to_string(),
            raw,
            data: if data.len() == 0 {
                None
            } else {
                Some(data)
            },
        })
    }
}

/// Check that account starts with appropriate numbers
/// and that it isn't excluded intentionally.
pub fn validate_acct(value: String) -> Result<String> {
    let acct = trim(value)?;
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
pub fn trim(value: String) -> Result<String> {
    Ok(value.trim().to_string())
}

/// Normalize the PrintKey value.
pub fn fix_printkey(value: String) -> Result<String> {
    let printkey = trim(value)?;
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
pub fn fix_meter_size(value: String) -> Result<String> {
    let meter_size = trim(value)?;
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
pub fn decode_special(value: String) -> Result<String> {
    let special = trim(value)?;
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
