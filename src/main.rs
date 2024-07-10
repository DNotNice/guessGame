use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess Game !");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is  : {}" , secret_number);
    
    loop {
    
    println!("Please input your guess :");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess : u32 = match guess.trim().parse(){
        Ok(num) => num ,
        Err(_) => continue,

    };
        println!("You Guessed {}" , guess);



    match guess.cmp(&secret_number) {
        Ordering::Less=> println!("{}","Too small".red()),
        Ordering::Equal=> {
            println!("{}" , "You win".green());
            break;
        },
        Ordering::Greater=> println!("{}","Too large".red()),
    }
    }

}

