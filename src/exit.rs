use ansi_term::Colour::Red;
use anyhow::Result;
use std::process::exit;

pub trait ExitOnError<T> {
    fn exit_on_error(self) -> T;
}

impl<T> ExitOnError<T> for Result<T> {
    fn exit_on_error(self) -> T {
        match self {
            Err(error) => {
                eprintln!("{}: {}", Red.bold().paint("error"), error);
                error
                    .chain()
                    .skip(1)
                    .for_each(|cause| eprintln!("{}: {}", Red.bold().paint("caused by"), cause));
                exit(1);
            }
            Ok(value) => value,
        }
    }
}
