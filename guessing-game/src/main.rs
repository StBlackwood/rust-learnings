use rand::random_range;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = random_range(1..100);
    println!("Please enter your guess:");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.trim().to_string();

        let guess3: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess3.cmp(&secret_number) {
            Ordering::Less => println!("Too small! {guess3}"),
            Ordering::Greater => println!("Too big! {guess3}"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }
}
