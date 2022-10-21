
#[derive(Debug)]
pub struct Calculator {
    result: i32,
}
impl Calculator {
    pub fn new() -> Calculator{
        Calculator {
            result : 0,
        }
    }
    pub fn add(&mut self, x: i32, y:i32) -> i32 {
        self.result = x + y;
        self.result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn calculator_addition(){
        let mut calculator = Calculator{result: 0};
        assert_eq!(calculator.add(5,7), 12);
    }
}