use reader::Reader;
use std::env;

mod reader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = args.get(1).expect(&format!(
        "Usage: {} <directory>",
        args[0].split("/").last().unwrap()
    ));

    let reader = Reader::new(dir);

    reader.read_folder();
}
