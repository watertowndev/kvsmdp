use std::fs;
use std::path::Path;
use crate::datarow::{DataRow, DataRowError};

#[derive(Debug)]
pub struct DataFile {
    rows: Vec<DataRow>,
    load_warnings: Vec<DataRowError>
}

#[derive(Debug)]
pub enum DataFileError {
    NonASCIIFile,
    FileError(std::io::Error)
}

type Result<T> = std::result::Result<T, DataFileError>;

impl DataFile {
    pub fn try_load(path: &Path) -> Result<DataFile> {
        let data = fs::read_to_string(path);
        if let Err(e) = data {
            return Err(DataFileError::FileError(e))
        }
        let data = data.unwrap();

        if !data.is_ascii() {
            return Err(DataFileError::NonASCIIFile);
        }

        let mut rows: Vec<DataRow> = vec![];
        let mut load_warnings: Vec<DataRowError> = vec![];

        for row in data.lines() {
            match DataRow::try_create(row) {
                Ok(r) => rows.push(r),
                Err(e) => load_warnings.push(e)
            }
        }

        Ok(DataFile{
            rows,
            load_warnings
        })
    }
}