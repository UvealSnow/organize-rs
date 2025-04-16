# Organize-rs

Organize-rs is a simple file organizer written in Rust. It helps you organize your files into folders based on their extensions. For example, if you have a folder with a lot of images, documents, and videos, you can use Organize-rs to move them into separate folders.

## Features
- Organizes files based on their extensions
- Supports custom folder names for different file types
- Can be run from the command line

## Installation

To install Organize-rs, you need to have Rust and Cargo installed on your system. You can install them by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).
Once you have Rust and Cargo installed, you can install Organize-rs by running the following command:

```bash
cargo install organize-rs
```

To use Organize-rs, navigate to the directory you want to organize and run the following command:

```bash
organize-rs
```

This will organize all the files in the current directory into folders based on their extensions.

## Options

You can customize the behavior of Organize-rs by using the following options:
- [ ] `--help`: Show help information
- [ ] `--version`: Show version information
- [ ] `--config`: Specify a custom configuration file, the program will look for a file named `organize.yml|yaml` in the current directory by default
- [ ] `--undo`: Undo the last organization operation, this works by taking the log of the last operation and moving the files back to their original locations.
- [ ] `--dry-run`: (false) Show what would be done without actually moving any files
- [ ] `--verbose`: (false) Show detailed output of the organization process.
- [ ] `--interactive`: (false) Ask for confirmation before moving files.
- [ ] `--exclude`: (null) Exclude certain file types from being organized. You can specify multiple file types separated by commas. For example: `--exclude jpg,png,mp4`
- [ ] `--stats`: (false) Show statistics about the organization process, including the number of files moved and the time taken.