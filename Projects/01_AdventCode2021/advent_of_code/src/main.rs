use advent_of_code::d01_sonar_sweep;
use std::time::{ Instant};
fn main() {
    let mut start = Instant::now();
    println!(
        "status sonar: {:?}",
        d01_sonar_sweep::sonar_sweep::depth_increase()
    );
    let mut duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);


    start = Instant::now();
    println!(
        "status sonar functional: {:?}",
        d01_sonar_sweep::sonar_sweep::depth_increase_functional()
    );
    duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);


    start = Instant::now();
    println!(
        "status sonar functional2: {:?}",
        d01_sonar_sweep::sonar_sweep::depth_increase_functional2()
    );
    duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
