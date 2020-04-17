//Lets us add method to structs
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub length: u32,
}

impl Rectangle {
    pub fn print_area(&self) {
        let area = self.length * self.width;
        println!("The area of a rectangle is {}", area)
    }
}