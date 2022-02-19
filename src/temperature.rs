use std::io;
use std::any::type_name;

pub fn run() {

    loop {
        println!("Enter number in Fahrenheit: ");

        let mut f = String::new();

        io::stdin()
            .read_line(&mut f)
            .expect("Failed to read line");

        match f.trim().parse::<i32>() {
            Ok(num) => {
                println!("Answer is {}", ((num-32)*5) / 9);
                break;
                // println!("{}", num)
            },
            Err(err) => {
                println!{"{}", err};
                continue;
            }
        };
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}