extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn fib(n: i32) -> i32 {
    if n <= 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    println!("Guess it!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {    
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Failed to read!");

        println!("Your guess is {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It's not a number, please input a REAL number~");
                continue
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You did it!");
                break
            },
        }
    }
    println!("Yes, the secret num is {}", secret_number);

    println!("fib(45) = {}", fib(45));
}

