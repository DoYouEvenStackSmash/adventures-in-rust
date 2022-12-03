// prelude

use std::io; // standard library
use rand::Rng;


fn main() {

    println!("please input your guess");
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // create a variable
    println!("the secret number is: {secret_number}");

    println!("please input your guess");

    let mut guess = String::new();

    let _apples = 5;  // immutable
    // let mut bananas = 5;    //mutable


    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    
    println!("you guessed: {guess}");

}
