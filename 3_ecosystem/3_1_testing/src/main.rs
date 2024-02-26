use std::{cmp::Ordering, env, io};

enum GuessResult {
    EnteredLess,
    EnteredGreater,
    Won,
}

fn game_logic(num: u32,secret_number: u32) -> GuessResult {
    match num.cmp(&secret_number) {
        Ordering::Less => GuessResult::EnteredLess, //("Too small!",false),
        Ordering::Greater => GuessResult::EnteredGreater,//("Too big!",false),
        Ordering::Equal => GuessResult::Won,//("You win!",true)
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = get_secret_number();

    loop {
        println!("Please input your guess.");

        let guess = match get_guess_number() {
            Some(n) => n,
            _ => continue,
        };

        println!("You guessed: {}", guess);

        match game_logic(guess, secret_number) {
            GuessResult::EnteredLess => println!("Too small!"),
            GuessResult::EnteredGreater => println!("Too big!"),
            GuessResult::Won => {
                println!("You win!");
                break;
            },
        }
    }
}

fn get_secret_number() -> u32 {
    let secret_number = env::args()
        .skip(1)
        .take(1)
        .last()
        .expect("No secret number is specified");
    secret_number
        .trim()
        .parse()
        .ok()
        .expect("Secret number is not a number")
}

fn get_guess_number() -> Option<u32> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::{game_logic, GuessResult};

    #[test]
    fn win() {
        let res = game_logic(10,10);
        assert!(matches!(res, GuessResult::Won));
    }

    proptest! {

        #[test]
        fn not_wining_randomly(secret in 0..1000u32,guess in 0..1000u32) {
            let res = game_logic(guess,secret);

            if secret == guess {
                prop_assert!(matches!(res, GuessResult::Won));
                
            } else {
                prop_assert!(matches!(res, GuessResult::EnteredGreater | GuessResult::EnteredLess));
            }
        }

    }
}