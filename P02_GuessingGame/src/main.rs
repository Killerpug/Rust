use std::io;            //library for user input
use rand::Rng;          //random crate
use std::cmp::Ordering; //comparison

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);   //ger_range is inclusive on the lower bound but exclusive on the upper bound.
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();          //mutable variable that is currently bound to a new, empty instance of a String

        io::stdin().read_line(&mut guess)       //Invoke io library and use read input method. Returns Result(ok, err) type
            .expect("Failed to read line");     //expect() should be called to avoid warnings

        //parse along with u32 enable us to convert the String type -> u32
        let guess: u32 = match guess.trim().parse()  { //converts the String into a real number. Allowing comparison with secret_number
            Ok(num) => num,                             //guess uses shadowing.trim eliminates white spaces.
            Err(_) => continue                        //match ensures that user entry is a number, _ means any
        };

        println!("You guessed: {}", guess);
         //match allows us to compare a value against a series of patterns and then execute code based on which pattern matches.
        match guess.cmp(&secret_number) {           //cmp compares 2 values & returns an enum(Less, Greater, Equial)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}