use crate::config::Message;
use anyhow::{Context, Result};
use owo_colors::{OwoColorize, Stream, Style};
use std::io::{Write, stderr};
use std::process::exit;

pub fn set_handler(message: Message) -> Result<()> {
    ctrlc::set_handler(move || {
        let mut stderr = stderr().lock();
        let interupt = message.interrupt.if_supports_color(Stream::Stderr, |text| {
            Style::new().yellow().bold().style(text)
        });
        let _ = writeln!(stderr, "\n\n{interupt}");
        exit(1);
    })
    .context("system error occurred")
}
