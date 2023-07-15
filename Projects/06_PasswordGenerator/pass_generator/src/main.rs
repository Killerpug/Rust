use std::char::from_u32;

use rand::prelude::*;
fn main() {
    let length:u32 = 10;
    generate_password(length);
}


fn generate_password(length: u32) {
    let mut password = String::new();
    let mut rng = rand::thread_rng();

    for letter in 0..length  {
        let random_char = rng.gen_range(0x30..=0x7D);
        let char= from_u32(random_char).expect("invalido");
        password.push(char);
    }

    println!("{}", password);
}