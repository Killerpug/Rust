use std::error::Error;
use std::fs;

pub struct SubmarinePosition {
    pub depth: u32,
    pub horizontal: u32,
    pub aim: u32,
}

impl SubmarinePosition {
    pub fn new() -> Self {
        Self {
            depth: 0,
            horizontal: 0,
            aim: 0,
        }
    }
    /// Receives directions in the form: "up x", "down x" or "forward x" and moves submarine.
    /// When going forward, aim affects how depth is increased:
    ///     ex. aim = 2 and forward = 3, increases depth by 6.
    /// up and down only affect aim
    pub fn command_parser(&mut self, direction: &str, amount: u32) {
        match direction {
            "up" => self.aim -= amount,
            "down" => self.aim += amount,
            "forward" => {
                self.horizontal += amount.clone();
                self.depth += self.aim.clone() * amount;
            }
            _ => println!("direction is invalid"),
        }
    }
    pub fn move_command(&mut self) -> Result<&mut SubmarinePosition, Box<dyn Error + 'static>> {
        let reader =
            fs::read_to_string("src/d02_direction_control/input/submarine_directions.txt")?;
        for line in reader.lines() {
            let mut current_line = line.split(" ");
            let direction = current_line.next().unwrap();
            let amount = current_line
                .next()
                .expect("fail obtain command")
                .parse::<u32>()
                .expect("failed to parse");
            self.command_parser(direction, amount);
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {}
}
