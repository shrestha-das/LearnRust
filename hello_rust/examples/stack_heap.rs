#![allow(unused)]

// Memory - stack and heap

// Stack 
// - Stores data of fixed size known at compile time
// - Fast
// - LIFO (Last In, First Out)

// Heap
// - Stores data of unknown size at compile time
// - Slower than stack
// - Memory safety is enforced through Rust's ownership and borrowing rules
fn main() {
    // Stack
    let x: i32 = 1;

    let arr: [i32; 10] = [1; 10];

    // Heap
    let mut s: String = "hello".to_string();
    s += "world";

    let mut v = vec![];
    v.push(0);
    v.push(0);
    v.push(0);

    let boxed = Box::new(1i32);
} 