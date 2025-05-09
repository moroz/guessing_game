use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::time::Instant;

fn main() {
    println!("Guess the number!");

    let mut count = 1;
    let secret_number = rand::rng().random_range(1..=100);

    let start_time = Instant::now();
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let elapsed = start_time.elapsed().as_secs_f64();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Time elapsed: {elapsed:.2}s"),
            Ordering::Greater => println!("Too big! Time elapsed: {elapsed:.2}s"),
            Ordering::Equal => break,
        }

        count += 1;
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    println!("You win in {count} attempts. Time elapsed: {elapsed:.2}s");
}
