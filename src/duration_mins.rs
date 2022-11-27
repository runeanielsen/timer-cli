use std::time::Duration;

pub trait DurationMins {
    fn from_mins(mins: u64) -> Duration;
    fn as_mins(&self) -> u64;
}

impl DurationMins for Duration {
    fn from_mins(mins: u64) -> Duration {
        Duration::from_secs(mins * 60)
    }

    fn as_mins(&self) -> u64 {
        self.as_secs() / 60
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_should_be_parsed_from_min_to_duration() {
        let assertions = vec![
            (0, Duration::from_secs(0)),
            (25, Duration::from_secs(1500)),
            (40, Duration::from_secs(2400)),
            (60, Duration::from_secs(3600)),
            (120, Duration::from_secs(7200)),
            (222, Duration::from_secs(13320)),
        ];

        for (param, expected) in assertions {
            assert_eq!(Duration::from_mins(param), expected);
        }
    }

    #[test]
    fn duration_is_converted_to_mins() {
        let assertions = vec![
            (Duration::from_mins(0), 0),
            (Duration::from_mins(1), 1),
            (Duration::from_mins(9), 9),
            (Duration::from_mins(25), 25),
            (Duration::from_mins(33), 33),
            (Duration::from_mins(111), 111),
        ];

        for (param, expected) in assertions {
            assert_eq!(param.as_mins(), expected);
        }
    }
}
