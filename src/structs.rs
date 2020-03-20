// Structs are custom data types lets you name & package together multiple related value
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

// method syntax in rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


//instantiating struct
pub fn area(rectangle: Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
