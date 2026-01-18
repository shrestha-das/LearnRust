#![allow(unused)]

// String and &str(string slice)
// - use "String" when you need ownership or mutability
// - use "&str" for read only string or string literals
fn main() {
    let msg: String = String::from("Hello Rust");
    let msg: String = "Hello Rust".to_string();

    let length: usize = msg.len();

    let s: &str = &msg[0..5];
    println!("msg: {}", msg);
    println!("length of msg: {}", length);
    println!("First 5 characters of msg String: {}", s);

    let s_slice : &str = "Hi hello!";
    let string = s_slice.to_string();
    println!("s_slice: {}", s_slice);
    println!("string: {}", string);

    // Rust will automatically converts &String into a &str but NOT vice versa
    let msg: String = String::from("Hello World");
    print(&msg);

    let s: &str = "Hello World";
    print(s);

    // Append &str to String
    let mut msg: String = String::from("Helloji!");
    msg += " How're you ji?";
    println!("The appended msg is: {}", msg);

    // String interpolation - format
    let lang = "Rust";
    let emoji = "🦀";
    let s = "Hello Rust 🦀";
    let mut s = "Hello".to_string();
    s += " ";
    s += lang;
    s += " ";
    s += emoji;
    println!("After so many +s : {s}");
    let x = format!("After formatting: Hello {} {}", lang, emoji);
    println!("{x}");
}

fn print(s: &str) {
    println!("{s}");
}