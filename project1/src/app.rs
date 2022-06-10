use crate::commands::*;
use clap::{ArgEnum, Parser};

#[derive(Parser)]
/// Declares every available command and option of the program.
pub struct Calculator {
    // TODO : with help
    #[clap(subcommand, arg_enum)]
    pub command: Command,
}

/// Help for release using terraform with chart helm
#[derive(Debug, ArgEnum, Parser)]
pub enum Command {
    // TODO : with help
    /// Check Version of providers and mooduls
    Multiplication(command_multiplication::MultiplicationOptions),
    /// Generates tab-completion script for your shell
    Addition(command_addition::AdditionOptions),
    Calculator(command_calculator::CalculatorOptions),
}