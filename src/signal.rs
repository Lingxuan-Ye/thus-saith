use crate::config::Messages;
use anyhow::{Context, Result};
use owo_colors::{OwoColorize, Stream, Style};
use std::io::{Write, stderr};
use std::process::exit;

pub fn set_handler(messages: Messages) -> Result<()> {
    ctrlc::set_handler(move || {
        let mut stderr = stderr().lock();
        let interupt = messages
            .interrupt
            .if_supports_color(Stream::Stderr, |text| {
                Style::new().yellow().bold().style(text)
            });
        let _ = writeln!(stderr, "\n\n{}", interupt);
        exit(1);
    })
    .context("system error occurred")
}
