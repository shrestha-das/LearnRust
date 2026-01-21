#![allow(unused)]

// When we need only one condition/case to be met
fn main() {
    let x: Option<i32> = Some(90);

    // 1. match
    match  x {
        Some(val) => println!("Option is {val}"),
        None => {}
    }

    // 2. if let
    if let Some(val) = x {
        println!("Option is {val}");
    }
} 