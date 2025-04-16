use config::Config;
use mover::Mover;
use reader::Reader;
use std::env;

mod config;
mod mover;
mod reader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).expect("Failed to parse command line arguments");

    let mover = Mover::new(Reader::new(config.target_dir).read_folder());

    let changes = mover.generate_change_log().join("\n");
    println!("{}", changes);

    if config.is_dry_run {
        return;
    }
}
