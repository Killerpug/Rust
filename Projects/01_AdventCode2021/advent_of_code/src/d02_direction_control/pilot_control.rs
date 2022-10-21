use std::error::Error;
use std::fs;

pub struct SubmarinePosition {
    pub depth: u32,
    pub horizontal: u32,
}

impl SubmarinePosition {
    pub fn new() -> Self {
        Self {
            depth: 0,
            horizontal: 0,
        }
    }
    /// Receives directions in the form: "up x", "down x" or "forward x" and moves submarine
    pub fn move_command(&mut self, direction: &str, amount: u32) {
        match direction {
            "up" => self.depth -= amount,
            "down" => self.depth += amount,
            "forward" => self.horizontal += amount,
            _ => println!("direction is invalid"),
        }
    }
}

pub fn calculate_position(submarine: &mut SubmarinePosition) -> Result<&mut SubmarinePosition, Box<dyn Error + 'static>> {

    let reader = fs::read_to_string("src/d02_direction_control/input/submarine_directions.txt")?;
    for line in reader.lines() {
        let mut current_line = line.split(" ");
        let direction = current_line.next().unwrap();
        let amount = current_line
            .next()
            .expect("fail obtain command")
            .parse::<u32>()
            .expect("failed to parse");
        submarine.move_command(direction, amount);
    }
    Ok(submarine)
}
