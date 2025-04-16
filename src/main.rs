use config::Config;
use mover::Mover;
use reader::Reader;
use std::{env, fs};

mod config;
mod mover;
mod reader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).expect("Failed to parse command line arguments");

    let target_dir = config.clone().target_dir;
    let mover = Mover::new(Reader::new(config.target_dir, config.exclude).read_folder());
    let changes = mover.generate_change_log().join("\n");

    if config.is_dry_run {
        return;
    }

    if config.is_verbose {
        println!("{}", changes);
    }

    if let Err(e) = mover.do_move_files() {
        println!("Failed with error {}", e);
    }

    if let Err(e) = fs::write(target_dir.join("changelog.txt"), changes) {
        println!("Failed to write changelog: {}", e);
    }
}
