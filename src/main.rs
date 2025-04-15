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
    let is_dry_run = true;

    let mover = Mover::new(Reader::new(dir).read_folder());

    let changes = mover.generate_change_log().join("\n");
    println!("{}", changes);

    if is_dry_run {
        return;
    }
}
