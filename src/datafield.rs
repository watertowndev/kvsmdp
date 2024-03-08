
#[derive(Debug)]
pub struct DataField {
    name: String,
    raw: String,
    data: String
}

#[derive(Debug)]
pub enum DataFieldError {
    StartOutOfRange(usize),
    EndOutOfRange(usize),
    StartAfterEnd,
    NonASCIIRow,
    InvalidAccountID,
    ExcludedAccountID,
    InvalidMeterSize,
    InvalidSpecialCode,
    BadNumber
}

type Result<T> = std::result::Result<T, DataFieldError>;

impl DataField {
    pub fn try_from_row(row: &str, name: &str,
                        start_idx: usize, end_idx: usize, post_fn: &dyn Fn(String) -> Result<String>)
                        -> Result<DataField>
    {
        if start_idx > row.len() {
            return Err(DataFieldError::StartOutOfRange(start_idx));
        }
        if end_idx > row.len() {
            return Err(DataFieldError::EndOutOfRange(end_idx));
        }
        if start_idx > end_idx {
            return Err(DataFieldError::StartAfterEnd);
        }
        if !row.is_ascii() {
            return Err(DataFieldError::NonASCIIRow);
        }

        let raw = row[start_idx..end_idx].to_string();
        let data = post_fn(raw.trim().to_string())?;

        Ok(DataField {
            name: name.to_string(),
            raw,
            data
        })
    }
}

/// Check that account starts with appropriate numbers
/// and that it isn't excluded intentionally.
pub fn validate_acct(value: String) -> Result<String> {
    let acct = trim(value)?;

    if acct.starts_with("54777") {
        Err(DataFieldError::ExcludedAccountID)
    }
    else {
        match &acct[0..2] {
            "51" | "52" | "53" | "54" => Ok(acct),
            _ => Err(DataFieldError::InvalidAccountID)
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
        _ => Err(DataFieldError::InvalidMeterSize)
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
        _ => Err(DataFieldError::InvalidSpecialCode)
    }
}

/// Trim leading zeroes.
pub fn trim_zeroes(value: String) -> Result<String> {
    let num = value.parse::<u32>();

    match num {
        Ok(x) => Ok(x.to_string()),
        Err(_) => Err(DataFieldError::BadNumber)
    }
}
