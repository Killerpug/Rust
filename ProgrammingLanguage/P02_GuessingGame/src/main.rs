use rand::Rng; //random crate
use std::cmp::Ordering;
use std::io; //library for user input //comparison

fn main() {
    println!("Guess the number!");
    //gen_range is inclusive on the lower bound but exclusive on the upper bound.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        //mutable variable that is currently bound to a new, empty instance of a String
        let mut guess = String::new();
        //Invoke io library and use read input method. Returns Result(ok, err) type
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //expect() should be called to avoid warnings
            //expect doesnt handle errors but logs when they happen.

        /* guess uses shadowing. parse along with u32 enable us to convert the String type -> u32
        converting the String into a real number and allowing comparison with secret_number.
        Trim() eliminates white spaces(and carrier returns \n) and match ensures that user entry is a number, _ means any */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        /*match allows us to compare a value against a series of patterns and then
        execute code based on which pattern matches.
        cmp() compares 2 values & returns a Result enum(Less, Greater, Equial) */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
