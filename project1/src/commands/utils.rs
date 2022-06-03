use std::time::Duration;
use std::thread::sleep;

#[derive(PartialEq, Eq)]
pub enum TriState {
    Empty,
    Exit,
    Fine,
}

const EMPTY: TriState = TriState::Empty;
const EXIT: TriState = TriState::Exit;
const FINE: TriState = TriState::Fine;

/// check the result on `res`
/// 
/// RETURN: 
/// - if the line is empty => `TriState::Empty`
/// - if he typed "exit" => `TriState::Exit`
/// - if all goes well& => `TriState::Fine`
///
/// 
pub fn check_res(res: i32, score: &mut i32, line: String) -> TriState {

    if line.is_empty() {
        return EMPTY;
    }
    if !line.bytes().all(|c| c.is_ascii_digit()) {
        if line == "exit" {
            println!("\n\x1b[90mGood bye see you soon !\x1b[0m\n");
            return EXIT;
        } else {
            println!("\n\x1b[93mOnly digit are allowed sorry :)\x1b[0m \nThe right answere was \x1b[31m{}\x1b[0m\n", res);
        }
    } else if line.parse::<i32>().unwrap() == res {
        *score += 1;
        println!("\n\x1b[92mNice your right !\x1b[0m\n");
    } else {
        println!("\n\x1b[91mOh no ! Wrong answere ...\x1b[0m \nThe right answere was \x1b[31m{}\x1b[0m\n", res);
    }
    sleep(Duration::new(0, 500000000));
    FINE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exemple() {
        let res = 12;
        let line: String = "".to_string();

        match check_res(res, &mut 0, line) {
            EMPTY => (),
            _ => panic!("Should return a TriState::Empty"),
        }
    }

    //TODO : test with match and test with asser_eq for FINE tristate
}
