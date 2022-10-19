use d01_sonar_sweep::ultrasonic_sensor;
use std::time::{ Instant};

pub mod d01_sonar_sweep{
    pub mod ultrasonic_sensor;
}

fn main() {
  day_01();
}

pub fn day_01() {
    let start = Instant::now();
    match  ultrasonic_sensor::depth_descent_slope() {
        Ok(value) =>  println!("status sonar: {:?}", value),
        Err(e) => println!("not okay: {:?}", e)
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);


}
