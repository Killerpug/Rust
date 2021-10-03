
use std::error::Error;  // Box<dyn Error> return a type that implements Error trait, does not habe to specify tparticular type, offers flexibility(dynamic Error).
use std::fs;        // read_to_string: put the entire contents of a file into a string.

/*Try to open file and propagate error if something went wrong*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;    // ? propagates error

    // println!("With text:\n{}", contents);
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents);
    };
    for line in  results {  // searching query in contents.
        println!("{}", line);
    }

    Ok(()) // we dont really need the result, just to know that the file is not corrupted.
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

/*Get a structure with the 2 user arguments*/
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 { 
            return Err("Must provide 2 arguments");
        }
        // @fix: clone is a little inefficient but let us OWN the data. Avoiding references and lifetimes.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_insensitive<'a>(query: &str,contents: &'a str,) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_line() {
        let query = "duct";
        let contents = "Rust: \n\
                        safe, fast, productive.\n\
                        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]

    fn test_search_sensitive() {
        let query = "duct";
        let contents = "Rust: \n\
                        safe, fast, productive.\n\
                        Pick three.\n\
                        Duct tape.";

        assert_ne!(vec!["Duct tape"], search(query, contents));
    }
    #[test]
    fn test_search_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\n\
                        safe, fast, productive.\n\
                        Pick three\n\
                        Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_insensitive(query, contents)
        );
    }
}


