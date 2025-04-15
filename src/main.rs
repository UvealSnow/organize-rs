use mover::Mover;
use reader::Reader;
use std::env;

mod mover;
mod reader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = args.get(1).expect(&format!(
        "Usage: {} <directory>",
        args[0].split("/").last().unwrap()
    ));

    let mover = Mover::new(Reader::new(dir).read_folder());
}
