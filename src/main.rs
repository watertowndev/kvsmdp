use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

use clap::{arg, command, value_parser};

use kvsmdp::datafield::DataFieldDef;
use kvsmdp::datafile::DataFile;

mod posthelp;

fn main() -> Result<(), Box<dyn Error>> {

    let matches = command!()
        .arg(arg!(<inputfile> "Input file to process").value_parser(value_parser!(PathBuf)))
        .arg(arg!(<outputfile> "Output file to create").value_parser(value_parser!(PathBuf)))
        .arg(arg!(<logfile> "Log file to write to").value_parser(value_parser!(PathBuf)))
        .get_matches();

    let input_file = matches.get_one::<PathBuf>("inputfile").expect("Input file is required.");
    let output_file = matches.get_one::<PathBuf>("outputfile").expect("Output file is required.");
    let log_file = matches.get_one::<PathBuf>("logfile").expect("Log file is required.");

    let row_defs = vec![
        DataFieldDef::new("AccountNo1", 0, 11, &posthelp::validate_acct),
        DataFieldDef::new("CyclNo1", 11, 16, &posthelp::cleanup),
        DataFieldDef::new("Status", 16, 23, &posthelp::cleanup),
        DataFieldDef::new("OwnerName", 23, 49, &posthelp::cleanup),
        DataFieldDef::new("PropAddrKey", 49, 60, &posthelp::cleanup),
        DataFieldDef::new("AddrLine2", 60, 86, &posthelp::cleanup),
        DataFieldDef::new("MeterID", 86, 100, &posthelp::validate_acct),
        DataFieldDef::new("CyclNo2", 100, 105, &posthelp::cleanup),
        DataFieldDef::new("ReadDigits", 105, 110, &posthelp::cleanup),
        DataFieldDef::new("No", 110, 114, &posthelp::cleanup),
        DataFieldDef::new("Type", 114, 119, &posthelp::cleanup),
        DataFieldDef::new("ARB", 119, 130, &posthelp::cleanup),
        DataFieldDef::new("FileKey", 130, 141, &posthelp::cleanup),
        DataFieldDef::new("StreetDirection", 141, 151, &posthelp::cleanup),
        DataFieldDef::new("StreetName", 151, 177, &posthelp::cleanup),
        DataFieldDef::new("StreetNumber", 177, 184, &posthelp::trim_zeroes),
        DataFieldDef::new("StreetUnit", 184, 191, &posthelp::cleanup),
        DataFieldDef::new("MeterSerial", 191, 205, &posthelp::cleanup),
        DataFieldDef::new("PrintKey", 205, 231, &posthelp::fix_printkey),
        DataFieldDef::new("MeterSize", 231, 237, &posthelp::fix_meter_size),
        DataFieldDef::new("Special", 237, 242, &posthelp::decode_special),
    ];
    let f = DataFile::try_load(input_file, &row_defs).unwrap();

    println!("{} entries, {} warnings", f.rows().len(), f.warnings().len());
    let output_fields = vec![
        "AccountNo1",
        "CyclNo1",
        "Status",
        "OwnerName",
        "PropAddrKey",
        "AddrLine2",
        "MeterID",
        "CyclNo2",
        "ReadDigits",
        "No",
        "Type",
        "ARB",
        "FileKey",
        "StreetDirection",
        "StreetName",
        "StreetNumber",
        "StreetUnit",
        "MeterSerial",
        "PrintKey",
        "MeterSize",
        "Special"
    ];

    let mut csv_rows = vec![];
    for r in f.rows() {
        let ordfields = r.get_ordered_fields(&output_fields).unwrap();
        let csvrow = ordfields.iter().map(|ff| ff.data()).collect::<Vec<String>>().join(",");
        csv_rows.push(csvrow);
    }

    println!("Found {} rows with {} warnings along the way, resulting in {} output rows.",
             f.rows().len(), f.warnings().len(), csv_rows.len());

    let outfile = File::create(output_file)?;
    for row in csv_rows {
        writeln!(&outfile, "{}", row)?;
    }

    let logfile = File::create(log_file)?;
    for w in f.warnings() {
        let ts = chrono::Local::now();
        writeln!(&logfile, "{} {:?}", ts.format("%Y-%m-%d %H:%M:%S %Z"), w)?;
    }

    Ok(())
}

