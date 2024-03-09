use std::fs;
use std::path::Path;
use crate::datarow::{DataRow, DataRowDef};
use crate::loadwarning::LoadWarning;

#[derive(Debug)]
pub struct DataFile {
    rows: Vec<DataRow>,
    load_warnings: Vec<LoadWarning>
}

#[derive(Debug)]
pub enum DataFileError {
    NonASCIIFile,
    FileError(std::io::Error)
}

type Result<T> = std::result::Result<T, DataFileError>;

impl DataFile {
    pub fn try_load(path: &Path, row_defs: &Vec<DataRowDef>) -> Result<DataFile> {
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

    pub fn rows(&self) -> &Vec<DataRow> {
        &self.rows
    }

    pub fn warnings(&self) -> &Vec<LoadWarning> {
        &self.load_warnings
    }
}