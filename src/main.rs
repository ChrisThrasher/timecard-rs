use chrono::prelude::*;
use clap::{command, Arg};
use regex::Regex;
use std::collections::HashMap;

fn am_pm_offset(timepoint: &str) -> i32 {
    let pm_regex = Regex::new(r".*pm").unwrap();
    if pm_regex.is_match(timepoint) {
        return 12;
    }
    let am_regex = Regex::new(r".*am").unwrap();
    if am_regex.is_match(timepoint) {
        return 0;
    }
    panic!("Found no am/pm suffix");
}

fn parse_timepoint(timepoint: &str) -> f32 {
    let mut timepoint = timepoint.to_string();

    let long_regex = Regex::new(r"([1][0-2]|[1-9]):[0-5][0-9](a|p)m").unwrap();
    if long_regex.is_match(timepoint.as_str()) {
        let offset = am_pm_offset(&timepoint);
        timepoint.truncate(timepoint.len() - 2);
        let hours = (timepoint[..timepoint.len() - 3].parse::<i32>().unwrap() % 12 + offset) as f32;
        let minutes = (timepoint[timepoint.len() - 2..].parse::<i32>().unwrap() as f32) / 60.0;
        return hours + minutes;
    }

    let short_regex = Regex::new(r"([1][0-2]|[1-9])(a|p)m").unwrap();
    if short_regex.is_match(timepoint.as_str()) {
        let offset = am_pm_offset(&timepoint);
        timepoint.truncate(timepoint.len() - 2);
        let hours = (timepoint.parse::<i32>().unwrap() % 12 + offset) as f32;
        return hours;
    }

    if timepoint == "now" {
        let now = Local::now();
        let midnight = now
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        let since_midnight = now.signed_duration_since(midnight).to_std().unwrap();
        let hours = since_midnight.as_secs_f32() / 3600.0;
        return hours;
    }

    panic!("Failed to parse \"{timepoint}\"");
}

fn calculate_durations(args: &[String]) -> HashMap<String, f32> {
    assert!(args.len() >= 3, "Must provide at least 3 arguments");
    assert!(args.len() % 2 == 1, "Must provide odd number of arguments");

    let mut durations = HashMap::<String, f32>::new();

    for i in (1..args.len()).step_by(2) {
        let duration = -parse_timepoint(&args[i - 1]) + parse_timepoint(&args[i + 1]);
        assert!(
            duration >= 0.0,
            "Duration of {duration} must be non-negative"
        );

        let label = &args[i];
        if durations.contains_key(label) {
            *durations.get_mut(label).unwrap() += duration;
        } else {
            durations.insert(label.to_string(), duration);
        }
    }

    durations
}

fn format_durations(durations: &HashMap<String, f32>) {
    let max_length = durations.keys().map(|label| label.len()).max().unwrap();
    let mut total: f32 = 0.0;
    let off_time: f32 = *durations.get("-").unwrap_or(&0.0);

    for (label, duration) in durations {
        if label == "-" {
            continue;
        }
        println!("{:max_length$}   {:.1} hours", label, duration);
        total += duration;
    }

    print!("\nTotal: {:.1} hours", total);

    if off_time > 0.0 {
        println!(" ({:.1} hours off)", off_time);
    } else {
        println!();
    }
}

fn main() {
    let mut cli = command!()
        .about(
            "Time formatting can follow one of two patterns depending on the time it
represents. 8:00 a.m. can be formatted as \"8:00am\" or \"8am\". 12:30 p.m. is
formatted only as \"12:30pm\". \"now\" is interpreted as the current time.

Any activities named \"-\" will be ignored. This activity's durations are
reported as \"off time\" should they exist.",
        )
        .arg(
            Arg::new("args")
                .help("Alternating times and activities")
                .num_args(1..)
                .required(true),
        )
        .get_matches();

    let args = cli.remove_many("args").unwrap().collect::<Vec<String>>();
    let durations = calculate_durations(&args);
    format_durations(&durations);
}

#[cfg(test)]
mod tests {
    mod am_pm_offset {
        use super::super::am_pm_offset;

        #[test]
        #[should_panic]
        fn empty_input() {
            am_pm_offset("");
        }

        #[test]
        #[should_panic]
        fn no_suffix() {
            am_pm_offset("12");
        }

        #[test]
        #[should_panic]
        fn garbage() {
            am_pm_offset("asdf;");
        }

        #[test]
        fn nominal() {
            assert_eq!(am_pm_offset("0am"), 0);
            assert_eq!(am_pm_offset("3am"), 0);
            assert_eq!(am_pm_offset("6am"), 0);
            assert_eq!(am_pm_offset("9am"), 0);
            assert_eq!(am_pm_offset("12am"), 0);

            assert_eq!(am_pm_offset("0pm"), 12);
            assert_eq!(am_pm_offset("3pm"), 12);
            assert_eq!(am_pm_offset("6pm"), 12);
            assert_eq!(am_pm_offset("9pm"), 12);
            assert_eq!(am_pm_offset("12pm"), 12);
        }
    }

    mod parse_timepoint {
        use super::super::parse_timepoint;

        #[test]
        #[should_panic]
        fn empty() {
            parse_timepoint("");
        }

        #[test]
        #[should_panic]
        fn no_suffix() {
            parse_timepoint("12");
        }

        #[test]
        #[should_panic]
        fn garbage() {
            parse_timepoint("qwerty");
        }

        #[test]
        #[should_panic]
        fn zero_am() {
            parse_timepoint("0am");
        }

        #[test]
        #[should_panic]
        fn zero_pm() {
            parse_timepoint("0pm");
        }

        #[test]
        fn nominal() {
            assert_eq!(parse_timepoint("12am"), 0.0);
            assert_eq!(parse_timepoint("3am"), 3.0);
            assert_eq!(parse_timepoint("6am"), 6.0);
            assert_eq!(parse_timepoint("9am"), 9.0);

            assert_eq!(parse_timepoint("12pm"), 12.0);
            assert_eq!(parse_timepoint("3pm"), 15.0);
            assert_eq!(parse_timepoint("6pm"), 18.0);
            assert_eq!(parse_timepoint("9pm"), 21.0);
        }
    }

    mod calculate_durations {
        use super::super::calculate_durations;
        use std::collections::HashMap;

        #[test]
        #[should_panic]
        fn too_few_args() {
            calculate_durations(&[]);
        }

        #[test]
        #[should_panic]
        fn odd_number_args() {
            calculate_durations(&vec![
                String::from("1pm"),
                String::from("work"),
                String::from("2pm"),
                String::from("gym"),
            ]);
        }

        #[test]
        fn nominal() {
            assert_eq!(
                calculate_durations(&vec![
                    String::from("9am"),
                    String::from("work"),
                    String::from("5pm"),
                ]),
                HashMap::<String, f32>::from([(String::from("work"), 8.0)])
            );
            assert_eq!(
                calculate_durations(&vec![
                    String::from("8am"),
                    String::from("work"),
                    String::from("12pm"),
                    String::from("lunch"),
                    String::from("1pm"),
                    String::from("work"),
                    String::from("5pm")
                ]),
                HashMap::<String, f32>::from([
                    (String::from("work"), 8.0),
                    (String::from("lunch"), 1.0)
                ])
            );
            assert_eq!(
                calculate_durations(&vec![
                    String::from("8am"),
                    String::from("work"),
                    String::from("12pm"),
                    String::from("-"),
                    String::from("1pm"),
                    String::from("work"),
                    String::from("5pm")
                ]),
                HashMap::<String, f32>::from([
                    (String::from("-"), 1.0),
                    (String::from("work"), 8.0)
                ])
            );
        }
    }
}
