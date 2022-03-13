use clap::Parser;
use text_io::read;
use std::process;
use rand::Rng;
use std::time::{Duration, Instant};
use std::thread::sleep;

/// Options for the `addition` command
/// 
/// `-t` to select the number of turn 
/// for the game
/// 
#[derive(Parser, Debug)]
pub struct AdditionOptions {
    #[clap(short = 't', long)]
    /// Search only the Helm resources in terraform files
    pub turn: Option<String>,
}

/// Randomise 3 diferents adition pattern, print it
/// and return the result of the operation as `i32`
/// 
/// At least a and b are use but c and d is used only
/// in certain case 
/// 
fn addition_patern(a: i32, b: i32, _c: i32, _d: i32) -> i32 {
    let pattern = rand::thread_rng().gen_range(0..3);
    let res: i32;
    if pattern == 0 {
        println!("{} + {}", a, b);
        res = a + b;
    } else if pattern == 1 {
        println!("{} + {} + {}", a, b, _c);
        res = a + b + _c;
    } else {
        println!("{} + {} + {} + {}", a, b, _c, _d);
        res = a + b + _c + _d;
    }
    res
}

/// Main function for addition loop
/// 
/// One cycle of the loop simulate one turn in the game
///
pub fn addition(param: AdditionOptions) {
    let mut turn = 10;
    if let Some(x) = param.turn {
        if x.bytes().all(|c| c.is_ascii_digit()) {
            turn = x.parse::<i32>().unwrap();
        }
    }

    println!("Ready to start ?");
    let line: String = read!("{}\n");
    let mut score = 0;

    if line == "Yes" || line == "yes" || line == "Oui" || line == "oui"  {
        println!("\nOK ! So let's start !\n");
    } else {
        process::exit(1);
    }
    let now = Instant::now();
    for n in 1..(turn + 1) {
        let a = rand::thread_rng().gen_range(0..200);
        let b = rand::thread_rng().gen_range(0..200);
        let c = rand::thread_rng().gen_range(0..200);
        let d = rand::thread_rng().gen_range(0..200);
        println!("\x1b[96mturn {}\x1b[0m", n);
        let res = addition_patern(a, b, c, d);
        let line: String = read!("{}\n");
        if !line.bytes().all(|c| c.is_ascii_digit()) {
            if line == "exit" {
                println!("\n\x1b[90mGood bye see you soon !\x1b[0m\n");
                process::exit(1);
            } else {
                println!("\n\x1b[93mOnly digit are allowed sorry :)\x1b[0m \nThe right answere was \x1b[31m{}\x1b[0m\n", res);
            }
        } else if line.is_empty() {
            println!("\n\x1b[93mOnly digit are allowed sorry :)\x1b[0m \nThe right answere was \x1b[31m{}\x1b[0m\n", res);
        } else if line.parse::<i32>().unwrap() == res {
            score += 1;
            println!("\n\x1b[92mNice your right !\x1b[0m\n");
        } else {
            println!("\n\x1b[91mOh no ! Wrong answere ...\x1b[0m \nThe right answere was \x1b[31m{}\x1b[0m\n", res);
        }
        sleep(Duration::new(0, 500000000));
    }
    println!("You scored {}/{} in {} seconds", score, turn, now.elapsed().as_secs());
}