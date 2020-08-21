fn main() {
    // Variable Scope in Rustlang
    let mut s = String::from("hello");
    s.push_str(", world");

    //considering two variables saved on stack
    let x = 5;
    let y = x;

    //Considering another variable
    let s1 = String::from("Hello World");
    let s2 = s1;

    println!("{}", s2);
}
