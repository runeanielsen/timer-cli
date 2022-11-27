use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Eq, PartialEq)]
pub struct CommandArguments {
    pub duration_min: u64,
    pub finished_script: Option<PathBuf>,
}

pub fn start_arguments(arguments: &[&str]) -> CommandArguments {
    let mut hm: HashMap<String, String> = HashMap::new();
    arguments.chunks_exact(2).for_each(|x| {
        hm.insert(x[0].to_string(), x[1].to_string());
    });

    CommandArguments {
        duration_min: match hm.get("-d") {
            Some(d) => d.parse().unwrap_or(25),
            None => 25,
        },
        finished_script: hm.get("-f").map(PathBuf::from),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_finished_script_argument() {
        let assertions = vec![
            (
                ["-f", "/home/my_awesome_finish_script"],
                CommandArguments {
                    duration_min: 25,
                    finished_script: Some(PathBuf::from("/home/my_awesome_finish_script")),
                },
            ),
            (
                ["-f", "/home/finish_script.sh"],
                CommandArguments {
                    duration_min: 25,
                    finished_script: Some(PathBuf::from("/home/finish_script.sh")),
                },
            ),
        ];

        for (param, expected) in assertions {
            assert_eq!(start_arguments(&param), expected);
        }
    }

    #[test]
    fn parse_arguments_with_flag_set() {
        let assertions = vec![
            (
                "5",
                CommandArguments {
                    duration_min: 5,
                    finished_script: None,
                },
            ),
            (
                "10",
                CommandArguments {
                    duration_min: 10,
                    finished_script: None,
                },
            ),
            (
                "15",
                CommandArguments {
                    duration_min: 15,
                    finished_script: None,
                },
            ),
            (
                "30",
                CommandArguments {
                    duration_min: 30,
                    finished_script: None,
                },
            ),
        ];

        for (param, expected) in assertions {
            let result = start_arguments(&["-d", param]);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn parse_start_arguments_with_no_duration_value_should_be_default() {
        let assertions = vec![
            (
                vec![],
                CommandArguments {
                    duration_min: 25,
                    finished_script: None,
                },
            ),
            (
                vec!["-d"],
                CommandArguments {
                    duration_min: 25,
                    finished_script: None,
                },
            ),
            (
                vec!["25"],
                CommandArguments {
                    duration_min: 25,
                    finished_script: None,
                },
            ),
            (
                vec!["25", "-d"],
                CommandArguments {
                    duration_min: 25,
                    finished_script: None,
                },
            ),
        ];

        for (param, expected) in assertions {
            assert_eq!(start_arguments(&param), expected);
        }
    }
}
