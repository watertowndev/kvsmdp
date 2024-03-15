#![warn(missing_docs)]
//! # ffreader
//! `ffreader` is a library built to facilitate processing of certain fixed-width
//! flat text files.
//!
//! It includes the following features:
//! - Field definitions based on column offset.
//! - Custom post-processing callbacks for each field.
//! - CSV and JSON output.

mod datafield;
mod datarow;
mod datafile;
mod loadwarning;

pub use datafield::DataField;
pub use datafield::DataFieldDef;
pub use datafield::DataFieldError;
pub use datafield::Result as DataFieldResult;

pub use datarow::DataRow;
pub use datarow::DataRowError;
pub use datarow::Result as DataRowResult;

pub use datafile::DataFile;
pub use datafile::DataFileError;
pub use datafile::Result as DataFileResult;

pub use loadwarning::LoadWarning;
