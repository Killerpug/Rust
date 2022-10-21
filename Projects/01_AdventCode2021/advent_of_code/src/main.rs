use std::io;

use d01_sonar_sweep::ultrasonic_sensor;

pub mod d01_sonar_sweep{
    pub mod ultrasonic_sensor;
}

fn main() {
    loop {
        let mut day = String::new();
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line"); //expect() logs errors

        /* guess uses shadowing. parse along with u32 enable us to convert the String type -> u32
        converting the String into a real number and allowing comparison with secret_number.
        Trim() eliminates white spaces(and carrier returns \n) and match ensures that user entry is a number, _ means any */
        let day: i32 = match day.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        run_day(day);
    }

}

pub fn day_01() {
    ultrasonic_sensor::depth_descent_slope().unwrap();
    ultrasonic_sensor::depth_descent_slope_slide_filter().unwrap();
}


pub fn run_day(num: i32) {
    match num {
        1 => day_01(),
        _ => println!("program not available")
    }
}