#![allow(unused)]

fn main() {
    // Match
    let x: i32 = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("others")
    }

    match x {
        1 | 2 | 3 => println!("one or two or three"),
        _ => println!("others")
    }

    match x {
        1..=10 => println!("within 1 to 10"), // *** 1..10 means within 1 to 9
        _ => println!("others")
    }

    match x {
        num @ 1..=10 => println!("within 1 to 10 and the number is {num}"),
        _ => println!("others")
    }

    let x: Option<i32> = Some(9);
    match x {
        Some(val) => println!("Option is {val}"),
        None => println!("None")
    }

    let x: Result<i32, String> = Ok(123);
    match x {
        Ok(val) => println!("Ok {val}"),
        Err(err) => println!("Err: {err}")
    }

    // Return 
    let x: Option<i32> = Some(9);

    let z: i32 = match x {
        Some(value) => value,
        None => 0
    };
    println!("match returned {z}");
}