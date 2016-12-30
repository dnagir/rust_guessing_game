extern crate rand;
use std::io::{self, BufRead};
use std::str::FromStr;
use rand::Rng;
use GuessResult::*;

#[derive(PartialEq, Debug, Clone, Copy)]
enum GuessResult {
    Smaller,
    Bigger,
    Guessed,
    BadInput,
}


fn process_guess(line: std::io::Result<String>, number_to_guess: i32) -> GuessResult {
    fn process_int(num: i32, number_to_guess: i32) -> GuessResult {
        match num {
            x if x < number_to_guess => Bigger,
            x if x > number_to_guess => Smaller,
            _ => Guessed,
        }
    }


    match line {
        Ok(line) => {
            i32::from_str(line.trim())
                .map(|num| process_int(num, number_to_guess))
                .unwrap_or(BadInput)
        }
        Err(_) => BadInput,
    }
}

fn format_guess(result: GuessResult) -> &'static str {
    match result {
        Smaller => "Try smaller",
        Bigger => "Try bigger",
        Guessed => "Spot on dude!",
        BadInput => "(didn't look like a number dude, try again)",
    }
}

fn game_over(result: GuessResult) -> bool {
    result == Guessed
}


fn main() {
    let input = io::stdin();
    let input = input.lock();

    println!("Guess a number");

    let number_to_guess = rand::thread_rng().gen_range(1, 1001);

    let guesses = input.lines()
        .map(|line| process_guess(line, number_to_guess))
        .take_while(|&guess_result| !game_over(guess_result))
        .map(format_guess)
        .map(|msg| println!("{}", msg))
        .count();

    println!("Guessed with {} attempts. Can you beat it?", guesses);
}

# [cfg(test)]
mod tests {
    use std::io::{Result, Error, ErrorKind};
    use super::{game_over, process_guess, format_guess};
    use super::GuessResult::*;

    # [test]
    fn game_over_is_true_when_guessed() {
        assert_eq!(game_over(Guessed), true);
    }

    # [test]
    fn game_over_is_false_if_not_guessed() {
        assert_eq!(game_over(Smaller), false);
        assert_eq!(game_over(Bigger), false);
        assert_eq!(game_over(BadInput), false);
    }

    # [test]
    fn process_guess_returns_a_guess_for_matching_number() {
        assert_eq!(Guessed, process_guess(ok_input("12"), 12));
        assert_eq!(Guessed, process_guess(ok_input("  123  "), 123));
        assert_eq!(Guessed, process_guess(ok_input(" -5 "), -5));
    }

    # [test]
    fn process_guess_returns_a_bigger_result() {
        assert_eq!(Bigger, process_guess(ok_input("40"), 50));
        assert_eq!(Bigger, process_guess(ok_input(" -5"), 50));
        assert_eq!(Bigger, process_guess(ok_input("-5 "), -4));
    }

    # [test]
    fn process_guess_returns_a_smaller_result() {
        assert_eq!(Smaller, process_guess(ok_input("50"), 40));
        assert_eq!(Smaller, process_guess(ok_input(" 5 "), -50));
        assert_eq!(Smaller, process_guess(ok_input(" 0 "), -1));
        assert_eq!(Smaller, process_guess(ok_input(" -5 "), -6));
    }

    # [test]
    fn process_guess_returns_bad_input_for_invalid_number() {
        assert_eq!(BadInput, process_guess(ok_input(" nay "), 0));
        assert_eq!(BadInput, process_guess(ok_input(" 123.55 "), 0));
        assert_eq!(BadInput, process_guess(ok_input("123f"), 0));
    }

    # [test]
    fn process_guess_returns_bad_input_for_failed_line_read() {
        assert_eq!(BadInput,
                   process_guess(error_input(ErrorKind::BrokenPipe), 0));
        assert_eq!(BadInput,
                   process_guess(error_input(ErrorKind::ConnectionAborted), 0));
        assert_eq!(BadInput,
                   process_guess(error_input(ErrorKind::InvalidData), 0));
    }

    # [test]
    fn format_guess_has_informative_message() {
        assert_eq!("Try smaller", format_guess(Smaller));
        assert_eq!("Try bigger", format_guess(Bigger));
        assert_eq!("Spot on dude!", format_guess(Guessed));
        assert_eq!("(didn't look like a number dude, try again)",
                   format_guess(BadInput));
    }

    fn ok_input(s: &str) -> Result<String> {
        Ok::<String, Error>(s.to_string())
    }

    fn error_input(kind: ErrorKind) -> Result<String> {
        Err(Error::new(kind, "Bang"))
    }

}
