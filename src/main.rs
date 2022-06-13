use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Enter your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Couldn't read string");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("This number is too small!"),
            Ordering::Greater => println!("This number is too large!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
