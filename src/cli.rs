use clap::{Arg, ArgMatches, ValueHint, command, value_parser};
use std::path::PathBuf;
use std::sync::LazyLock;

pub static MATCHES: LazyLock<ArgMatches> = LazyLock::new(|| {
    command!()
        .args([
            Arg::new("mean")
                .long("mean")
                .value_name("NUMBER")
                .value_parser(value_parser!(f64))
                .help("Average time per character (unit: ms)"),
            Arg::new("stddev")
                .long("stddev")
                .value_name("NUMBER")
                .value_parser(value_parser!(f64))
                .help("Standard deviation of time per character (unit: ms)"),
            Arg::new("config")
                .long("config")
                .value_name("FILE")
                .value_hint(ValueHint::FilePath)
                .value_parser(value_parser!(PathBuf))
                .help("Load the specified configuration file"),
        ])
        .get_matches()
});
