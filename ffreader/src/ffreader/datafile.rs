use std::fs;
use std::path::Path;
use crate::ffreader::DataFieldDef;
use crate::ffreader::DataRow;
use crate::ffreader::LoadWarning;

pub struct DataFile {
    rows: Vec<DataRow>,
    load_warnings: Vec<LoadWarning>
}

#[derive(Debug)]
pub enum DataFileError {
    NonASCIIFile,
    FileError(std::io::Error)
}

pub type Result<T> = std::result::Result<T, DataFileError>;

impl DataFile {
    pub fn try_load(path: &Path, row_defs: &Vec<DataFieldDef>) -> Result<DataFile> {
        let data = fs::read_to_string(path);
        if let Err(e) = data {
            return Err(DataFileError::FileError(e))
        }
        let data = data.unwrap();

        if !data.is_ascii() {
            return Err(DataFileError::NonASCIIFile);
        }

        let mut rows: Vec<DataRow> = vec![];
        let mut load_warnings: Vec<LoadWarning> = vec![];

        for (row_num, row) in data.lines().enumerate() {
            match DataRow::try_create(row, row_defs) {
                Ok(r) => rows.push(r),
                Err(e) => load_warnings.push(LoadWarning::new(row_num, e))
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