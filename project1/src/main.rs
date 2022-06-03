#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::nursery,
    nonstandard_style
)]

//! calculator
use crate::app::*;
use crate::commands::*;
use clap::Parser;

mod app;
mod commands;

fn main() {
    env_logger::try_init().ok();
    let app: Calculator = Calculator::parse();


    match app.command {
        // TODO : with help
        Command::Multiplication(params) => command_multiplication::multiplication(params),
        Command::Addition(params) => command_addition::addition(params),
    };
}