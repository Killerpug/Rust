// count the number of times depth measurement increases from previous measurement.

use std::fs;
use std::fs::File;
use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

pub const MAX: i32 = i32::MAX;
pub const MIN: i32 = i32::MAX;
const NUMBER_OF_MEASUREMENTS: usize = 3;
const MAX_MEASUREMENTS: u32 = 3;

/// count the number of times a depth measurement increases from the previous measurement
///
/// sensor measurements shall be available in file input/01_sensor_measurements.txt
pub fn depth_descent_slope() -> Result<i32, &'static str> {
    let mut count_increased_depth = 0;
    let mut previous_line:i32 = MAX;    // avoids wrongly increasing count on first value

    // open file and attach a buffer
    let file = match File::open("src/d01_sonar_sweep/input/01_sensor_measurements.txt") {
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
                continue
            },
        };
        if previous_line < next_line { // increased depth
            count_increased_depth += 1;
        }
        previous_line = next_line;
    }

    println!("the depth increased: {} times", count_increased_depth);
    return Ok(count_increased_depth);
}

/// depth sonar implemented with functional rust: Iterators
pub fn depth_descent_slope_functional() -> Result<i32, Box<dyn Error + 'static>> {
    let mut count_increased_depth = 0;
    let mut previous_line:i32 = MAX;    // avoids wrongly increasing count on first value

    let reader = fs::read_to_string("src/d01_sonar_sweep/input/01_sensor_measurements.txt")?;
    for line in reader.lines(){
        let next_line =  match line.trim_end().parse::<i32>() {
            Ok(line) => line,
            _ => {
                println!("type is not integer, skiping line");
                continue;
            },
        };

        if previous_line < next_line { // increased depth
            count_increased_depth += 1;
        }
        previous_line = next_line;
    }
    println!("the depth increased: {} times", count_increased_depth);
    Ok(count_increased_depth)
}

/// depth sonar implemented with functional rust: Iterators and closures
pub fn depth_descent_slope_functional2() -> Result<i32, Box<dyn Error + 'static>> {
    let mut count_increased_depth = 0;
    let mut previous_line:i32 = MAX;    // avoids wrongly increasing count on first value

    let reader = fs::read_to_string("src/d01_sonar_sweep/input/01_sensor_measurements.txt")?;
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
    println!("the depth increased: {} times", count_increased_depth);
    Ok(count_increased_depth)
}

/// functional2 depth sonar with 3-measurement slide filter
pub fn depth_descent_slope_slide_filter() -> Result<i32, Box<dyn Error + 'static>> {
    let mut count_increased_depth = 0;
    let mut previous_avg:i32 = MAX;    // avoids wrongly increasing count on first value
    let mut filter_depth = SlideFilter::new();
    let mut pos = 0;
    let mut result;
    let mut new_avg;

    let reader = fs::read_to_string("src/d01_sonar_sweep/input/01_sensor_measurements.txt")?;
    let split = reader.lines()
        .map(|s|s.parse::<i32>()
            .unwrap_or(MIN));
    for line in split{
        result = filter_depth.calculate_avg(line, pos % MAX_MEASUREMENTS);
        pos += 1;
        match result{
            Some(avg) => new_avg = avg,
            None => continue,
        }
        if previous_avg < new_avg { // increased depth
            count_increased_depth += 1;
        }
        previous_avg = new_avg;

    }
    println!("the depth increased: {} times", count_increased_depth);
    Ok(count_increased_depth)
}


#[derive(Debug)]
pub struct SlideFilter {
    slide_window: HashMap<u32, i32>,
    measurement: Option<i32>,
}
impl SlideFilter{
    pub fn new() -> SlideFilter {
        let map: HashMap<u32, i32> = HashMap::with_capacity(NUMBER_OF_MEASUREMENTS);
        let measure:Option<i32> = None;
        // default slide filter
        SlideFilter{
            slide_window: map,
            measurement: measure,
        }
    }
    pub fn calculate_avg(&mut self, sample: i32, position: u32) -> Option<i32>{
        let old_value = self.slide_window.insert(position, sample);
        let mut sum = 0;

        match self.measurement{
            // filter is working, calculate most recent sum on window(updating only 1 value)
            Some(avg) => sum = avg - old_value.unwrap() + sample,
            None => {
                if self.slide_window.len() != NUMBER_OF_MEASUREMENTS { // too few samples
                    return None;
                } else { // calculate average for the first time
                    for val in self.slide_window.values() {
                        sum += val;
                    }
                }
            },
        }
        self.measurement = Some(sum);
        self.measurement
    }
    pub fn remove_sample(&mut self, position: u32) {
        self.slide_window.remove_entry(&position);
        self.measurement = None;
    }
}



#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    /// speed comparison between:
    /// - iterator with buffer reader implementation(inefficient access due to generation of syscalls)
    /// - Iterator with fs implementation(efficient)
    /// - Iterators + closures
    fn chronometric() {
        // show duration with $cargo test -- --nocapture

        let start = Instant::now();
        match  depth_descent_slope() {
            Ok(value) =>  println!("sonar slope: {:?}", value),
            Err(e) => println!("not okay: {:?}", e)
        }
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);

        let start = Instant::now();
        match  depth_descent_slope_functional() {
            Ok(value) =>  println!("sonar slope: {:?}", value),
            Err(e) => println!("not okay: {:?}", e)
        }
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);

        let start = Instant::now();
        match  depth_descent_slope_functional2() {
            Ok(value) =>  println!(" sonar slope: {:?}", value),
            Err(e) => println!("not okay: {:?}", e)
        }
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);

        let start = Instant::now();
        match  depth_descent_slope_slide_filter() {
            Ok(value) =>  println!("sonar slope: {:?}", value),
            Err(e) => println!("not okay: {:?}", e)
        }
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}
