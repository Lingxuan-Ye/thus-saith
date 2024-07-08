use clap::{command, value_parser, Arg, ArgMatches, Command};

pub struct Cli(Command);

impl Cli {
    pub fn new() -> Self {
        let command = command!().args([
            Arg::new("mean")
                .long("mean")
                .value_name("NUMBER")
                .value_parser(value_parser!(f64))
                .default_value("100")
                .help("The average time (in milliseconds) taken per character"),
            Arg::new("std-dev")
                .long("std-dev")
                .value_name("NUMBER")
                .value_parser(value_parser!(f64))
                .default_value("100")
                .help("The standard deviation (in milliseconds) of the time taken per character"),
        ]);
        Self(command)
    }

    pub fn get_matches(self) -> ArgMatches {
        self.0.get_matches()
    }
}
