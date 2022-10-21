use std::io;

use d01_sonar_sweep::ultrasonic_sensor;
use d02_direction_control::pilot_control;

pub mod d01_sonar_sweep {
    pub mod ultrasonic_sensor;
}
pub mod d02_direction_control {
    pub mod pilot_control;
}
fn main() {
    loop {
        let mut day = String::new();
        println!("select a program to run between 1-30");
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

pub fn run_day(num: i32) {
    match num {
        1 => day_01(),
        2 => day_02(),
        _ => println!("program not available"),
    }
}

pub fn day_01() {
    ultrasonic_sensor::depth_descent_slope().unwrap();
    ultrasonic_sensor::depth_descent_slope_slide_filter().unwrap();
}

pub fn day_02() {
    match pilot_control::calculate_position() {
        Ok(submarine_pos) => {
            println!(
                "depth: {}, heading: {}, multiplied {}",
                submarine_pos.depth,
                submarine_pos.horizontal,
                submarine_pos.depth * submarine_pos.horizontal
            )
        }
        Err(e) => println!("error {}", e),
    }
}
