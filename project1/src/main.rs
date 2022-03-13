#![forbid(unsafe_code)]

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
        Command::Soustraction(params) => command_soustraction::soustraction(params),
        Command::Addition(params) => command_addition::addition(params),
    };
}