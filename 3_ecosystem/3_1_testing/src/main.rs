use std::{cmp::Ordering, env, io};

fn main() {
    println!("Guess the number!");

    let secret_number = get_secret_number();


    // #Please input your guess.
    // #...
    // #You guessed: {}
    // -# You win!
    // -# Too small!
    // -# Too big!

    loop {
        println!("Please input your guess.");

        let guess = match get_guess_number() {
            Some(n) => n,
            _ => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
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



// We test an invalid input.
// We test consistency on same number
// We somehow test ability to win?
// - should i write the guesser program?
// ####
// fuzzer may not win a game
// mock what?
// e2e tests may go