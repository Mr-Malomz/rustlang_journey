pub fn owner() {
    let mut s1 = String::from("Hello");
    let _len = calculate_length(&s1); //& means referecing the type of an initial value
    let _word = first_word(&s1);
    s1.clear();

    println!("the length is {}.", s1.len());
}

pub fn calculate_length(s: &String) -> usize {
    return s.len();
}

pub fn first_word(s: &String) {
    let _bytes = s.as_bytes();
    let _s = String::from("Hello world");
}

// STRING SLICES
//are reference to  part of a String  type (Heap Memory type)
// let s = String::from("Hello world");

// let hello = &s[0..5];  //referencing where holle covers
// let world = &s[6..11]; //referencing where world covers
