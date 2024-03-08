use kvsmdp::datafile::DataFile;

fn main() {
    println!("Hello, world!");
    let f = DataFile::try_load("c:/temp/test.txt".as_ref()).unwrap();


    println!("{} entries, {} warnings", f.rows().len(), f.warnings().len());
    for w in f.warnings() {
        println!("{}", w);
    }
}
