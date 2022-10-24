use std::path::PathBuf;

pub struct Config {
    pub base_dir_path: PathBuf,
    pub time_entry_path: PathBuf,
    pub finished_path: PathBuf,
}

impl Config {
    pub fn new(base_path: PathBuf) -> Config {
        let base_dir_path: PathBuf = [base_path, PathBuf::from(".config/timer-cli")]
            .iter()
            .collect();

        let time_entry_path: PathBuf = [&base_dir_path, &PathBuf::from("time_entry")]
            .iter()
            .collect();

        let finished_path: PathBuf = [&base_dir_path, &PathBuf::from("finished")]
            .iter()
            .collect();

        Config {
            base_dir_path,
            time_entry_path,
            finished_path,
        }
    }
}
