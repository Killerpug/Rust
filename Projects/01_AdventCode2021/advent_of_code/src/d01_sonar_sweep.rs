// count the number of times depth measurement increases from previous measurement.

pub mod sonar_sweep{
    use std::fs;
    use std::fs::File;
    use std::error::Error; 
    use std::i32::{MAX, MIN};
    use std::io::{prelude::*, BufReader};

    pub fn depth_increase() -> Result<(), &'static str> {
        let mut count_increased_depth = 0;
        let mut previous_line:i32 = MAX;    // avoids wrongly increasing count on first value
        
        // open file and attach a buffer
        let file = match File::open("src/input/01_sweep_report.txt") {
            Ok(file) => file,
            _ => return Err("file not found"),
        };
        let reader = BufReader::new(file);
        
        // get Strings from file and parse it to integers for comparison
        for line in reader.lines(){
            let next_line =  match line.unwrap().trim_end().parse::<i32>() {
                Ok(line) => line,
                _ => {
                    println!("type is not integer, skiping line");
                    MIN
                },
            };
            if next_line == MIN {   //skip this value
                continue;
            }
            if previous_line < next_line { // increased depth
                count_increased_depth += 1;
            }
            previous_line = next_line;
        }

        println!("the deepth increased: {} times", count_increased_depth);
        Ok(())
    }

    // depth sonar implemented with functional rust: Iterators 
    pub fn depth_increase_functional() -> Result<(), Box<dyn Error + 'static>> {
        let mut count_increased_depth = 0;
        let mut previous_line:i32 = MAX;    // avoids wrongly increasing count on first value
        
        let reader = fs::read_to_string("src/input/01_sweep_report.txt")?;
        for line in reader.lines(){
            let next_line =  match line.trim_end().parse::<i32>() {
                Ok(line) => line,
                _ => {
                    println!("type is not integer, skiping line");
                    MIN
                },
            };
            if next_line == MIN {   //skip this value
                continue;
            }
            if previous_line < next_line { // increased depth
                count_increased_depth += 1;
            }
            previous_line = next_line;
        }
        println!("the deepth increased: {} times", count_increased_depth);
        Ok(())
    }

    // depth sonar implemented with functional rust: Iterators and closures
    pub fn depth_increase_functional2() -> Result<(), Box<dyn Error + 'static>> {
        let mut count_increased_depth = 0;
        let mut previous_line:i32 = MAX;    // avoids wrongly increasing count on first value
        
        let reader = fs::read_to_string("src/input/01_sweep_report.txt")?;
        let split = reader.lines()
            .map(|s|s.parse::<i32>()
            .unwrap_or(MIN));
        for line in split{
            if line == MIN {   //skip this value
                continue;
            }
            if previous_line < line { // increased depth
                count_increased_depth += 1;
            }
            previous_line = line;
        }
        println!("the deepth increased: {} times", count_increased_depth);
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
