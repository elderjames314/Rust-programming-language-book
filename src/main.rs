use core::num;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number !");
    println!("Please input the guess number !");

    let num_range = 1..101;

    let secret_number = rand::thread_rng().gen_range(num_range);

   // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(num) => num,
            Err(_) => continue
        };

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guess : {}", guess);

        match guess.cmp(&secret_number) {
            
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win");
                break;
            }

        }


    }

}
