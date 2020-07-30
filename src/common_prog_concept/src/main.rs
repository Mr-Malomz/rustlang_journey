fn main() {
    // Variables and immutability in rust
    let x: i8 = 3;
    println!("{}", x);

    //declaring constant
    const _PI: f32 = 3.142;

    //shadowing
    let y = 5;
    let y = y + 1;
    println!("The new value of y is {}", y);

    //TUPLE IN RUST
    let _tup: (i32, f64, u8) = (500, 6.5, 1);
    let tup2 = (800, 7.8, 5);

    //destructuring tuples
    let (o, p, q) = tup2;
    println!("The values in this tupple are {}, {} & {}", o, p, q);

    //Accessing tuple using the period notation and corresponding index
    let tuple_index = tup2.1;
    println!("{}", tuple_index);

    //ARRAYS IN RUST
    let array1 = [1, 2, 3, 4];

    //accessing array element
    let _second = array1[1];

    //condition
    // if 1 > array1.len() {
    //     println!("greater than 1")
    // }

    //loop (WHILE)
    let mut numb = 3;
    // while numb != array1[3] {
    //     println!("Still passing number {}", numb);

    //     numb = numb - 1;
    // }

    //loop (forIn)
    for item in array1.iter() {
        println!("the value is: {}", item)
    }
}
