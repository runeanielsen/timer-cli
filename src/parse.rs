use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct StartArguments {
    pub duration_min: u32,
}

pub fn start_arguments(arguments: &[&str]) -> StartArguments {
    let mut hm: HashMap<String, String> = HashMap::new();
    arguments.chunks_exact(2).for_each(|x| {
        hm.insert(x[0].to_string(), x[1].to_string());
    });

    let duration_min = match hm.get("-d") {
        Some(d) => d.parse().unwrap_or(25),
        None => 25,
    };

    StartArguments { duration_min }
}

#[test]
fn parse_start_arguments_with_duration_value() {
    let assertions = vec![
        ("5", StartArguments { duration_min: 5 }),
        ("10", StartArguments { duration_min: 10 }),
        ("15", StartArguments { duration_min: 15 }),
        ("30", StartArguments { duration_min: 30 }),
    ];

    for assertion in assertions {
        let result = start_arguments(&["-d", assertion.0]);
        assert_eq!(result, assertion.1);
    }
}

#[test]
fn parse_start_arguments_with_no_duration_value_should_be_default() {
    assert_eq!(start_arguments(&[]), StartArguments { duration_min: 25 });

    assert_eq!(
        start_arguments(&["-d"]),
        StartArguments { duration_min: 25 }
    );

    assert_eq!(
        start_arguments(&["25"]),
        StartArguments { duration_min: 25 }
    );

    assert_eq!(
        start_arguments(&["25", "-d"]),
        StartArguments { duration_min: 25 }
    );
}
