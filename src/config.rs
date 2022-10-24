use std::{env, path::PathBuf};

pub fn dir_path() -> PathBuf {
    #[allow(deprecated)]
    build_config_path(&env::home_dir().unwrap())
}

fn build_config_path(base_path: &PathBuf) -> PathBuf {
    [base_path, &PathBuf::from(".config/timer-cli")]
        .iter()
        .collect()
}

#[test]
fn build_finished_path_() {
    let result = build_config_path(&PathBuf::from("/home/notation"));
    assert_eq!(result, PathBuf::from("/home/notation/.config/timer-cli"));
}
