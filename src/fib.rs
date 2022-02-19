use std::io;

pub fn run() {
    loop {
        let mut fib1 = 1;
        let mut fib2 = 1;

        let mut n = String::new();

        println!("Enter n of fibonacci: ");

        io::stdin()
            .read_line(&mut n)
            .expect("Error reading line");

        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let mut i = 0;
        while i < n - 2 {
            let mut fib_sum = fib1 + fib2;
            fib1 = fib2;
            fib2 = fib_sum;
            i+=1;
        }

        println!("Anwser is {}", fib2);
        break;
    }
}