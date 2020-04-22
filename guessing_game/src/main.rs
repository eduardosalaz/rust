use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main (){

    let secret_num = rand::thread_rng().gen_range(1, 10); //Generate a pseudorandom within the range 
    
    println!("Guess the number I am thinking of."); //println! is a macro, the ! tells us so
    println!("I am thinking of a number that is between 1 and 10.");
    println!("Can you guess it?");
    
    loop {
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess) //This is a reference to the mutable variable guess
            .expect("Sorry, didn't quite catch that.");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };
        
        println!("Your guess is: {}", guess); //The {} are syntactic sugar for variables, there are many, many way to use them
        
        //As we can see, Rust has a lot of functional programming influence, mainly Haskell and ML.
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You won!");
                println!("The number that I was thinking of is {}", secret_num);
                break;
            }
        }
}
}
