use std::fmt::{Display, Formatter};
use ffreader::{DataFieldError, DataFieldResult};

pub enum PostError{
    InvalidOrExcludedAccountID(String),
    InvalidMeterSize(String),
    InvalidSpecialCode(String,)
}
impl Display for PostError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PostError::InvalidOrExcludedAccountID(a) => format!("Invalid or Excluded Account ID ({})", a),
            PostError::InvalidMeterSize(m) => format!("Invalid Meter Size ({})", m),
            PostError::InvalidSpecialCode(c) => format!("Invalid Special Code ({}):", c),
        };
        write!(f, "{}", s)
    }
}

/// Check that the account starts with appropriate numbers
/// and that it isn't excluded intentionally.
pub fn validate_acct(value: String) -> DataFieldResult<String> {
    let acct = cleanup(value)?;
    if acct.len() < 10 {
        Err(DataFieldError::Problem(Box::new(PostError::InvalidOrExcludedAccountID(acct))))
    }
    else if acct.starts_with("54777") {
        Err(DataFieldError::Problem(Box::new(PostError::InvalidOrExcludedAccountID(acct))))
    }
    else {
        match &acct[0..2] {
            "51" | "52" | "53" | "54" => Ok(acct),
            _ => Err(DataFieldError::Problem(Box::new(PostError::InvalidOrExcludedAccountID(acct))))
        }
    }
}

/// Remove whitespace from the beginning and end of value.
/// Converts ampersands to 'and' and commas to spaces.
pub fn cleanup(value: String) -> DataFieldResult<String> {
    if value.contains("\"") {
        Err(DataFieldError::FieldContainsQuote(value))
    }
    else {
        Ok(value.trim().replace("&", "and").replace(",", " "))
    }
}

/// Normalize the PrintKey value.
pub fn fix_printkey(value: String) -> DataFieldResult<String> {
    let printkey = cleanup(value)?;
    let mut npk: String;
    if printkey.contains("NOT") {
        Ok(printkey)
    }
    else if printkey.len() == "0-0000-000.000".len() {
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
pub fn fix_meter_size(value: String) -> DataFieldResult<String> {
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
        ""  => Ok("".to_string()),
        _ => Err(DataFieldError::Problem(Box::new(PostError::InvalidMeterSize(meter_size))))
    }
}

/// Decode the Special value into expanded terms.
pub fn decode_special(value: String) -> DataFieldResult<String> {
    let special = cleanup(value)?;
    match special.as_str() {
        "S" => Ok("Shut".to_string()),
        "E" => Ok("Elderly Exemption".to_string()),
        "X" => Ok("Exempt".to_string()),
        "O" => Ok("Outside User".to_string()),
        "R" => Ok("Removed".to_string()),
        _ => Err(DataFieldError::Problem(Box::new(PostError::InvalidSpecialCode(special))))
    }
}

/// Trim leading zeroes. Will erase all zeroes if nothing else is present.
/// On failure or negative, returns an Ok(empty string).
pub fn trim_zeroes(value: String) -> DataFieldResult<String> {
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
