use kvsmdp::datafile::DataFile;

fn main() {
    println!("Hello, world!");
    let f = DataFile::try_load("c:/temp/test.txt".as_ref());

    println!("{:?}", f);
}
