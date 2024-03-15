use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use crate::DataFieldDef;
use crate::DataRow;
use crate::LoadWarning;

/// Holds a list of DataRows and a list of the LoadWarnings
/// encountered during creation.
pub struct DataFile {
    rows: Vec<DataRow>,
    load_warnings: Vec<LoadWarning>
}

/// Errors that DataFiles may encounter.
#[derive(Debug)]
pub enum DataFileError {
    /// Non-ASCII characters were encountered.
    NonASCIIFile,
    /// A file I/O error.
    FileError(PathBuf, std::io::Error)
}

impl Display for DataFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DataFileError::NonASCIIFile => "Non ASCII file.".to_string(),
            DataFileError::FileError(p, e) => format!("IO error on {} ({})", p.to_string_lossy(), e.to_string())
        };
        write!(f, "Data File Error: {}", s)
    }
}

impl Error for DataFileError { }

/// Convenient Result shorthand for DataFileError Results.
pub type Result<T> = std::result::Result<T, DataFileError>;

impl DataFile {
    /// Attempt to load a file and parse its rows and fields.
    /// Non-fatal issues are included in the DataFile as a list of LoadWarnings.
    pub fn try_load(path: &Path, row_defs: &Vec<DataFieldDef>) -> Result<DataFile> {
        let data = fs::read_to_string(path);
        if let Err(e) = data {
            return Err(DataFileError::FileError(path.into(), e))
        }
        let data = data.unwrap();

        if !data.is_ascii() {
            return Err(DataFileError::NonASCIIFile);
        }

        let mut rows: Vec<DataRow> = vec![];
        let mut load_warnings: Vec<LoadWarning> = vec![];

        for (line_index, row) in data.lines().enumerate() {
            match DataRow::try_create(row, row_defs) {
                Ok(r) => rows.push(r),
                Err(e) => load_warnings.push(LoadWarning::new(line_index, Box::new(e)))
            }
        }

        Ok(DataFile{
            rows,
            load_warnings
        })
    }

    /// Get a reference to the rows contained in the DataFile.
    pub fn rows(&self) -> &Vec<DataRow> {
        &self.rows
    }

    /// Get a reference to the list of warnings generated during creation.
    pub fn warnings(&self) -> &Vec<LoadWarning> {
        &self.load_warnings
    }

    /// Generator a json version of the data
    /// This function works for this specific data; no guarantees elsewhere.
    pub fn jsonify(&self) -> String {
        let mut json_row_list = vec![];
        for row in &self.rows {
            let mut json_row = String::from("{");
            let mut kv_list = vec![];
            for field in row.fields() {
                kv_list.push(format!("\"{}\": \"{}\"", field.name(), field.data()));
            }
            json_row.push_str(kv_list.join(",").as_str());
            json_row.push('}');
            json_row_list.push(json_row);
        }

        format!("[{}]", json_row_list.join(",\n"))
    }
}