use clap::{Arg, ArgMatches, Command, ValueHint, command, value_parser};
use std::path::PathBuf;

pub struct Cli(Command);

impl Cli {
    pub fn new() -> Self {
        let command = command!().args([
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
        ]);
        Self(command)
    }

    pub fn get_matches(self) -> ArgMatches {
        self.0.get_matches()
    }
}
