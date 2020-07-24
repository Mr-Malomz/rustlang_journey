fn main() {
    // Variables and immutability in rust
    let x: i8 = 3;
    println!("{}", x);

    //declaring constant
    const PI: f32 = 3.142;

    //shadowing
    let y = 5;
    let y = y + 1;
    println!("The new value of y is {}", y);
}
