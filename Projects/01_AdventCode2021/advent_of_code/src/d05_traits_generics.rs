// 1. create a figure that accepts, unsigned int  values to create the object, 
// for example designer::new("Box", height = 5, width = 3); 
// 2. Implement the "area" trait, so that we can request let box_area = designer.area();

use std::ops::Mul;
#[derive(Debug, Eq, PartialEq)]
struct Designer {
    object: String,
    height: u32,
    width: u32,
}

impl Designer {
    fn new(object: String, height: u32, width: u32) -> Self {
        if object == "Box".to_string() {
            Self {
                object,
                height,
                width,
            }
        } else {
            Self {
                object: "empty".to_string(),
                height,
                width,
            }
        }
    }
}

impl Area for Designer {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

trait Area {
    fn area(&self) -> u32;
}



pub fn figures() {
    let square = Designer::new("Box".to_string(), 6, 3);
    let square = square.area();
    println!("{:?}", square);
}


#[cfg(test)]
mod tests {
    use crate::d05_traits_generics::{Designer, Area};

    #[test]
    fn area() {
        let square = Designer::new("Box".to_string(), 5, 3);
        
        assert_eq!(square.area(), 15);
    }
}