use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    let mut guess = String::new();

   let res =  io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("Guessed number strLen: {}", res);

    println!("You guessed: {}", guess);
}