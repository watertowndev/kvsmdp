use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Contains a datafield, including name, raw data, and processes data (if any).
#[derive(Debug, Clone)]
pub struct DataField {
    name: String,
    raw: String,
    data: Option<String>
}

/// Errors that DataFields may encounter.
pub enum DataFieldError {
    /// The field specification has the starting index after the ending one.
    StartAfterEnd(String),
    /// The field contains non-ASCII characters.
    NonASCII(String),
    /// Problem occurred: used for application-specific post-processing errors.
    Problem(Box<dyn ToString>),
    /// The field contains a quotation mark (").
    FieldContainsQuote(String)
}

impl Display for DataFieldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DataFieldError::StartAfterEnd(i) => format!("Start index is after end ({})", i),
            DataFieldError::NonASCII(f) => format!("Non ASCII ({})", f),
            DataFieldError::Problem(p) =>  format!("Problem: {}", p.to_string()),
            DataFieldError::FieldContainsQuote(f) => format!("Field contains quote ({})", f)
        };
        write!(f, "{}", s)
    }
}

impl Debug for DataFieldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error for DataFieldError { }

/// Convenient Result shorthand for DataFieldError Results.
pub type Result<T> = std::result::Result<T, DataFieldError>;

/// Holds details pertaining to the structure of a row and the desired post-processing function.
pub struct DataFieldDef<'a> {
    /// The name of the field.
    pub name: String,
    /// The start index (0-based column) of the field
    pub start_idx: usize,
    /// The end index of the field (exclusive end)
    /// e.g., ABC123 with start index 0 and end index 3 yields "ABC"
    pub end_idx: usize,
    /// Function to execute on the field after loading.
    /// This function's output will affect the data stored and can return a
    /// DataFieldError to facilitate validation.
    pub post_process: &'a dyn Fn(String) -> Result<String>
}

impl Display for DataFieldDef<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}-{} ", self.name, self.start_idx, self.end_idx)
    }
}

impl DataFieldDef<'_> {
    /// Convenience function to instantiate a new DataFieldDef with the provided data.
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
    /// Instantiate a new DataField. Panics if bad conditions occur,
    /// such as non-ASCII data or quotes in the string.
    pub fn new(name: &str, data: String) -> DataField {
        assert!(data.is_ascii());
        assert!(!data.contains("\""));
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

    /// Try to create a DataField from a row using the provided data field definition.
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
            return Err(DataFieldError::NonASCII(field_def.name.to_string()));
        }

        let raw = row[field_def.start_idx..end_idx].to_string();
        let data = (field_def.post_process)(raw.trim().to_string())?;

        if data.contains("\"") {
            return Err(DataFieldError::FieldContainsQuote(data));
        }

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

    /// Obtain a reference to the name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Obtain an unwrapped clone of the data. An empty string is returned if None.
    pub fn data(&self) -> String {
        self.data.clone().unwrap_or("".to_string())
    }

    /// Obtain a reference to the raw data.
    pub fn raw(&self) -> &String {
        &self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn echo_ok(s: String) -> Result<String> {
        Ok(s)
    }

    #[test]
    fn non_ascii_caught() {
        let test_row = String::from("(╯°□°）╯︵ ┻━┻");
        let def = DataFieldDef::new("testname", 0, test_row.len(), &echo_ok);

        let r = DataField::try_from_row(&test_row, &def);
        match r.unwrap_err() {
            DataFieldError::NonASCII(_) => {}
            _ => panic!()
        }
    }

    #[test]
    fn stray_quote_caught() {
        let test_row = String::from("test \" quote");
        let def = DataFieldDef::new("testname", 0, test_row.len(), &echo_ok);

        let r = DataField::try_from_row(&test_row, &def);
        match r.unwrap_err() {
            DataFieldError::FieldContainsQuote(_) => {}
            _ => panic!()
        }
    }

    #[test]
    fn range_checks_work() {
        let test_row = String::from("test field");
        let def = DataFieldDef::new("testname", 28, 50, &echo_ok);
        let r = DataField::try_from_row(&test_row, &def);
        assert!(r.unwrap().data.is_none());

        let def = DataFieldDef::new("testname", 7, 5, &echo_ok);
        let r = DataField::try_from_row(&test_row, &def);
        match r.unwrap_err() {
            DataFieldError::StartAfterEnd(_) => {}
            _ => panic!()
        }
    }

    #[test]
    fn fields_extracted() {
        let test_row = String::from("1234567890  test1 test2  x");
        let defs = vec![
            DataFieldDef::new("field1", 0, 10, &echo_ok),
            DataFieldDef::new("field2", 11, 17, &echo_ok),
            DataFieldDef::new("field3", 18, 24, &echo_ok),
            DataFieldDef::new("field4", 25, 27, &echo_ok),
            ];
        let fields = vec! [
            "1234567890", "test1", "test2", "x"
        ];

        for (def, field) in defs.iter().zip(fields) {
            let r = DataField::try_from_row(&test_row, &def).unwrap();
            assert_eq!(r.data.unwrap(), field);
        }
    }
}