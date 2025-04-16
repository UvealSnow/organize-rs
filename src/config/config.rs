use std::path::PathBuf;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub exclude: Vec<String>,
    pub target_dir: PathBuf,
    config_file_path: Option<PathBuf>,
    pub is_dry_run: bool,
    pub is_stats_run: bool,
    pub is_verbose: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Self, String> {
        let mut exclude = Vec::new();
        let mut config_file_path = None;
        let mut is_dry_run = false;
        let mut is_stats_run = false;
        let mut is_verbose = false;

        let mut iter = args.into_iter().skip(1);
        let target_dir = match iter.next() {
            None => PathBuf::from("."),
            Some(t) => PathBuf::from(t),
        };

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--dry-run" => is_dry_run = true,
                "--stats" => is_stats_run = true,
                "--verbose" => is_verbose = true,
                "--exclude" => {
                    let patterns = iter
                        .next()
                        .ok_or("--exclude requires a pattern (e.g. '*.rs,*.toml').")?;
                    exclude.extend(patterns.split(",").map(|s| s.to_string()));
                }
                "--config" => {
                    let path_as_string = iter
                        .next()
                        .ok_or("--config requires a path to a config file.")?;
                    config_file_path = Some(PathBuf::from(path_as_string));
                }
                _ => return Err(format!("unknown argument: {}", arg)),
            }
        }

        Ok(Config {
            exclude,
            target_dir,
            config_file_path,
            is_dry_run,
            is_stats_run,
            is_verbose,
        })
    }
}
