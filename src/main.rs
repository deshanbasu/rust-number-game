use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;



fn main(){

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,11);

    println!("Your secret number is {}", secret_number);
    println!("Please input your guess..");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to readline");

    let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");

    loop {
        
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }unimplemented!();
    }

}