use minigrep;
use minigrep::Config;
use std::env; // args: Returns the arguments that this program was started with (passed via the command line).
use std::process; // exit program

fn main() {
    /*Accepting and storing command line arguments*/
    // args() returns a Struct that implements Iterator trait which chaings a series of values
    // collect() turns an Iterator into a collection such as a vector containing all elements produced by the iterator.
    // Note: Collect needs type annotation,":<String>" in this case, because collect can create many kinds of collections.
    let args: Vec<String> = env::args().collect(); // read Command Line arguments and store them into a vector.
    // store String to search and filename into a structure. Unwrap if okay, else pass inner error(defined in Config::new()).
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);    //eprintln prints to stderr
        process::exit(1);
    });
    /* try to read the file. we are not interested in the result, so no unwrap is required.*/
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
