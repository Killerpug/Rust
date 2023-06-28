use std::io;
use std::process::exit;

mod d01_sonar_sweep;
mod d02_direction_control;

fn main() {
    loop {
        let mut day = String::new();
        println!("select a program to run between 1-30 or 0 to exit");
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line"); //expect() logs errors

        let day: i32 = match day.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input");
                continue;
            }
        };
        run_day(day);
    }
}

pub fn run_day(num: i32) {
    match num {
        0 => exit(1),
        1 => day_01(),
        2 => day_02(),
        _ => println!("program not available"),
    }
}

pub fn day_01() {
    d01_sonar_sweep::depth_descent_slope().unwrap();
    d01_sonar_sweep::depth_descent_slope_slide_filter().unwrap();
}

pub fn day_02() {
    let mut submarine = d02_direction_control::SubmarinePosition::new();
    match d02_direction_control::SubmarinePosition::move_command(&mut submarine) {
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
