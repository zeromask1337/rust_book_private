use rand::Rng;
use std::io;
use std::cmp::Ordering;

use std::any::type_name;

pub fn run() {


    println!("Guess the number!");

    let secret_number = rand::thread_rng()
    .gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            },
        };

        println!("{}", type_of(guess));

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}