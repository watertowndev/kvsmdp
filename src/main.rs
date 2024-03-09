use kvsmdp::datafile::DataFile;

fn main() {
    println!("Hello, world!");
    let f = DataFile::try_load("c:/temp/test.txt".as_ref()).unwrap();

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

