use clap::{Arg, ArgMatches, Command, ValueHint, command, value_parser};
use std::path::PathBuf;

pub fn build_command() -> Command {
    command!().args([
        Arg::new("mean")
            .long("mean")
            .value_name("NUMBER")
            .value_parser(value_parser!(f64))
            .default_value("100")
            .help("Average time per character (unit: ms)"),
        Arg::new("std-dev")
            .long("std-dev")
            .value_name("NUMBER")
            .value_parser(value_parser!(f64))
            .default_value("100")
            .help("Standard deviation of time per character (unit: ms)"),
        Arg::new("config")
            .long("config")
            .value_name("FILE")
            .value_hint(ValueHint::FilePath)
            .value_parser(value_parser!(PathBuf))
            .help("Load the specified configuration file"),
    ])
}

pub struct Args<'a> {
    pub mean: f64,
    pub std_dev: f64,
    pub config: Option<&'a PathBuf>,
}

impl<'a> Args<'a> {
    pub fn from_matches(matches: &'a ArgMatches) -> Self {
        let Some(&mean) = matches.get_one::<f64>("mean") else {
            unreachable!()
        };
        let Some(&std_dev) = matches.get_one::<f64>("std-dev") else {
            unreachable!()
        };
        let config = matches.get_one::<PathBuf>("config");
        Self {
            mean,
            std_dev,
            config,
        }
    }
}
