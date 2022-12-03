// prelude

use std::io; // standard library
use rand::Rng;
use std::cmp::Ordering;


fn main() {

    println!("please input your guess");
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // create a variable
    println!("the secret number is: {secret_number}");



    let _apples = 5;  // immutable
    // let mut bananas = 5;    //mutable
    // infinite loop
    loop {
        // convert string to int 
        let mut guess = String::new();

        println!("please input your guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = guess.trim().parse().expect("Not a number");
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
