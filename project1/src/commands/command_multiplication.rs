use clap::Parser;
use text_io::read;
use std::process;
use rand::Rng;
use std::time::Instant;
use crate::utils::*;

/// Options for the `outdated` command
#[derive(Parser, Debug)]
pub struct MultiplicationOptions {
    #[clap(short = 't', long)]
    /// number of turn, 10 by default
    pub turn: Option<String>,
}

const EMPTY: TriState = TriState::Empty;
const EXIT: TriState = TriState::Exit;
const FINE: TriState = TriState::Fine;

/// Print a tab that contain the source, actual version, latest version and path location
///
pub fn multiplication(param: MultiplicationOptions) {
    let mut turn = 10;
    if let Some(x) = param.turn {
        if x.bytes().all(|c| c.is_ascii_digit()) {
            turn = x.parse::<i32>().unwrap();
        }
    }
    let mut score = 0;

    println!("Ready to start ?");
    let line: String = read!("{}\n");

    if line == "Yes" || line == "yes" || line == "Oui" || line == "oui"  {
        println!("\nOK ! So let's start !\n");
    } else {
        process::exit(1);
    }
    let now = Instant::now();
    for n in 1..(turn + 1) {

        let a = rand::thread_rng().gen_range(1..9);
        let b = rand::thread_rng().gen_range(1..9);

        println!("\x1b[96mturn {}\x1b[0m", n);

        println!("{} x {}", a, b);
        let res = a * b;
        let line: String = read!("{}\n");
        match check_res(res, &mut score, line) {
            EXIT => process::exit(1),
            EMPTY => println!("Hmm you should enter your responce before press enter!\n\x1b[91mSadly you lose a turn ...\x1b[0m\n"),
            FINE => continue,
        }
    }
    println!("You scored {}/{} in {} seconds", score, turn, now.elapsed().as_secs());
}