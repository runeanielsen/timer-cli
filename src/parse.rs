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
    let x = vec!["-d", "30"];
    let result = start_arguments(&x);

    let exp = StartArguments { duration_min: 30 };

    assert_eq!(result, exp);
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
