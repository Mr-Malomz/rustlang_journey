fn main() {
    let mut s1 = String::from("Hello");
    let _len = calculate_length(&s1); //& means referecing the type of an initial value
    let word = first_word(&s1);
    s1.clear();

    println!("the length is {}.", word);
}

fn calculate_length(s: &String) -> usize {
    return s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    return s.len();
}
