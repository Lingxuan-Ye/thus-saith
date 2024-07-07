use crate::sanity::{NonNegFinite, PosFinite};
use anyhow::{anyhow, Result};
use clap::{command, Arg, ArgMatches, Command};

pub struct Cli(Command);

impl Cli {
    pub fn new() -> Self {
        let command = command!().args([
            Arg::new("mean")
                .short('m')
                .long("mean")
                .value_name("NUMBER")
                .value_parser(|s: &str| -> Result<PosFinite> {
                    let value: f64 = s.parse().map_err(|_| anyhow!("invalid literal"))?;
                    PosFinite::build(value)
                })
                .default_value("2000")
                .help("The average number of characters typed per minute"),
            Arg::new("std-dev")
                .short('s')
                .long("std-dev")
                .value_name("NUMBER")
                .value_parser(|s: &str| -> Result<NonNegFinite> {
                    let value: f64 = s.parse().map_err(|_| anyhow!("invalid literal"))?;
                    NonNegFinite::build(value)
                })
                .default_value("4000")
                .help("The standard deviation of the number of characters typed per minute"),
        ]);

        Self(command)
    }

    pub fn get_matches(self) -> ArgMatches {
        self.0.get_matches()
    }
}
