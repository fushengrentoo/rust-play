extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Faild to read line");
//    println!("The secret number is {}", secret_number);
        println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse()
            {
                Ok(num) => num,
                Err(_) => continue
            };
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