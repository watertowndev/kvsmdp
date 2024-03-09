use kvsmdp::datafield;
use kvsmdp::datafield::DataFieldDef;
use kvsmdp::datafile::DataFile;


fn main() {
    println!("Hello, world!");
    let mut row_defs = vec![];
    row_defs.push(DataFieldDef::new("AccountNo1", 0, 11, &datafield::validate_acct));
    row_defs.push(DataFieldDef::new("CyclNo1", 11, 16, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("Status", 16, 23, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("OwnerName", 23, 49, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("PropAddrKey", 49, 60, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("AddrLine2", 60, 86, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("MeterID", 86, 100, &datafield::validate_acct));
    row_defs.push(DataFieldDef::new("CyclNo2", 100, 105, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("ReadDigits", 105, 110, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("No", 110, 114, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("Type", 114, 119, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("ARB", 119, 130, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("FileKey", 130, 141, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("StreetDirection", 141, 151, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("StreetName", 151, 177, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("StreetNumber", 177, 184, &datafield::trim_zeroes));
    row_defs.push(DataFieldDef::new("StreetUnit", 184, 191, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("MeterSerial", 191, 205, &datafield::cleanup));
    row_defs.push(DataFieldDef::new("PrintKey", 205, 231, &datafield::fix_printkey));
    row_defs.push(DataFieldDef::new("MeterSize", 231, 237, &datafield::fix_meter_size));
    row_defs.push(DataFieldDef::new("Special", 237, 242, &datafield::decode_special));

    let f = DataFile::try_load("c:/temp/test.txt".as_ref(), &row_defs).unwrap();

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

    println!("{:?}", csv_rows);
}

