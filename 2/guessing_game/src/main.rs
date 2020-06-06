extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"); // read_lineがio::Result型（実態はenum）を返し、Result型がErrのときクラッシュさせる

    let guess: u32 = guess.trim().parse()
        .expect("please type a number");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("match");
                break;
        }
    }
    println!("you guessed: {}", guess);
    }

}
