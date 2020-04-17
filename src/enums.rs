// Enum are use to define types of various pattern
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrWithVal {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message<T> {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    Some(T), //this are Options in rust; they're used to check if variable is null or not
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

//instance
fn instanceOfEnum(coin: Coin) -> u32 {
    let four = IpAddrKind::V4;
    let five: &str;
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

