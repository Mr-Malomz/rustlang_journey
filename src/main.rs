mod ownership;
mod structs;

fn main() {
    let rect1 = structs::Rectangle {
        width: 30,
        height: 40,
    };
    println!("The area of given rectangle is {}", structs::area(rect1));
}
