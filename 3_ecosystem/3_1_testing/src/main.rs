use std::{cmp::Ordering, env, io};


fn game_logic(num: u32,secret_number: u32) -> (&'static str,bool) {
    match num.cmp(&secret_number) {
        Ordering::Less => ("Too small!",false),
        Ordering::Greater => ("Too big!",false),
        Ordering::Equal => ("You win!",true)
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

        let (s,f) = game_logic(guess,secret_number);

        println!("{s}");
        if f { break; }
    }
}


// we mock this 
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

    use crate::game_logic;

    #[test]
    fn win() {
        let (_,won) = game_logic(10,10);
        assert!(won);
    }

    proptest! {

        #[test]
        fn not_wining_randomly(secret in 0..1000u32,guess in 0..1000u32) {
            let (_,flag) = game_logic(guess,secret);

            if secret != guess {
                prop_assert!(flag == false);
                
            } else {
                prop_assert!(flag == true);
            }
        }

    }
}