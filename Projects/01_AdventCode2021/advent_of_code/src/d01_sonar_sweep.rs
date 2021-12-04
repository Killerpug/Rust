// count the number of times a depth measurement increases from previous measurement.

pub mod sonar_sweep{
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    pub fn depth_increase() {
        let mut count_increased_depth = 0;
        let mut actual_line= String::new();
        let file = File::open("01_sweep_report.txt").expect("rip");
        let mut reader = BufReader::new(file);
        
        reader.read_line(&mut actual_line).unwrap();
        let mut actual_line: i32 = actual_line.trim_end().parse().unwrap();
        for line in reader.lines(){
            let line = line.unwrap();
            let mut next_line: i32 = line.trim_end().parse().unwrap();
            if actual_line < next_line {
                count_increased_depth += 1;
            }
            actual_line = next_line; 
         }
         println!("the deepth increased: {} times", count_increased_depth);
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
