mod calculator_module;  // import module

use calculator_module::Calculator;

fn main() {
    
    let mut calculator_1 = Calculator::new();
    println!("addition: {}", Calculator::add(&mut calculator_1, 4,5));
    
}
