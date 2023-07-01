use enigo::*;
use std::{thread, time};
fn main() {

    let mut enigo = Enigo::new();
    enigo.key_click(Key::Meta);
    let delay = time::Duration::from_millis(2000);
    thread::sleep(delay);
    enigo.key_sequence("libreOffice Calc");
    thread::sleep(delay);
    enigo.key_click(Key::Return);
    
}
