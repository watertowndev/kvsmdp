use std::fmt::{Display, Formatter};

/// Contains a datafield, including name, raw data, and processes data (if any).
#[derive(Debug, Clone)]
pub struct DataField {
    name: String,
    raw: String,
    data: Option<String>
}

/// Errors that DataFields may encounter.
pub enum DataFieldError {
    StartAfterEnd(String),
    NonASCII(String),
    Problem(Box<dyn ToString>),
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

pub type Result<T> = std::result::Result<T, DataFieldError>;

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

    //Returns a reference to the name.
    pub fn name(&self) -> &String {
        &self.name
    }

    //returns a clone of the data. An empty string is returned if None.
    pub fn data(&self) -> String {
        self.data.clone().unwrap_or("".to_string())
    }

    //returns a reference to the raw data.
    pub fn raw(&self) -> &String {
        &self.raw
    }
}