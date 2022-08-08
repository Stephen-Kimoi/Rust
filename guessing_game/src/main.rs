use std::io; 
use std::cmp::Ordering; 
use rand::Rng; 

fn main() {
    println!("====loading====="); 

    let secret_number = rand::thread_rng().gen_range(1..=101); 

    println!("Welcome to guess the number game!"); 
    
    loop {
        println!("Please input your guess"); 

        let mut guess = String::new(); 

        io::stdin() 
            .read_line(&mut guess) 
            .expect("Failed to read line"); 

        println!("You guessed {guess}"); 

        let guess: u32 = guess.trim().parse().expect("Kindly input a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Opps! Too small"), 
            Ordering::Equal => {
                println!("Congratulations! It matches"); 
                break; 
            },  
            Ordering::Greater => println!("Opps! Too big")
        }
        
    }


}