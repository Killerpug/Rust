
use std::env;       // args: Returns the arguments that this program was started with (passed via the command line).
use std::process;   // exit program
mod lib;
use lib::Config;

fn main() {
    // iterators produce a series of values 
    // and collect turns it into a collection such as vector containing all elements produced by the iterator.
    // Note: Collect needs type annotation(String in this case) because collect can create many kinds of collections.
    /*Accepting and storign command line arguments*/
    let args: Vec<String> = env::args().collect();                      // read Command Line arguments.
    let config: Config = Config::new(&args).unwrap_or_else(|err| {      // store String to search and filename into a structure. Unwrap if okay, else pass inner error.
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    /* try to read the file. we are not interested in the result, so no unwrap is required.*/
    if let Err(e) = lib::run(config) {   
        println!("Application error: {}", e);
        process::exit(1);
    }
    
}




