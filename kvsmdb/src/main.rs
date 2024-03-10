use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

use clap::{arg, command, value_parser};

use kvsmdp::ffreader::DataFieldDef;
use kvsmdp::ffreader::DataFile;

mod posthelp;

fn main() -> Result<(), Box<dyn Error>> {

    let matches = command!()
        .arg(arg!(<inputfile> "Input file to process").value_parser(value_parser!(PathBuf)))
        .arg(arg!(<csvfile> "Output CSV file to create").value_parser(value_parser!(PathBuf)))
        .arg(arg!(<jsonfile> "Output JSON file to create").value_parser(value_parser!(PathBuf)))
        .arg(arg!(<logfile> "Log file to write to").value_parser(value_parser!(PathBuf)))
        .get_matches();

    let input_file = matches.get_one::<PathBuf>("inputfile").expect("Input file is required.");
    let csv_file = matches.get_one::<PathBuf>("csvfile").expect("Output file is required.");
    let json_file = matches.get_one::<PathBuf>("jsonfile").expect("Output file is required.");
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
    let data_file = DataFile::try_load(input_file, &row_defs).unwrap();

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
    for r in data_file.rows() {
        if let Ok(ordfields) = r.get_ordered_fields(&output_fields) {
            let csvrow = ordfields.iter().map(|ff| ff.data()).collect::<Vec<String>>().join(",");
            csv_rows.push(csvrow);
        }
    }

    println!("Found {} rows with {} warnings along the way, resulting in {} output rows.",
             data_file.rows().len(), data_file.warnings().len(), csv_rows.len());
    println!("Check log file for details.");

    let outfile_csv = File::create(csv_file)?;
    for row in csv_rows {
        writeln!(&outfile_csv, "{}", row)?;
    }

    let outfile_json = File::create(json_file)?;
    write!(&outfile_json, "{}", data_file.jsonify())?;

    let logfile = File::create(log_file)?;
    for w in data_file.warnings() {
        let ts = chrono::Local::now();
        writeln!(&logfile, "{} {}", ts.format("%Y-%m-%d %H:%M:%S %Z"), w)?;
    }

    Ok(())
}



