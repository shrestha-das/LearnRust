#![allow(unused)]

// Generic Datatypes -
// Option, Result, Vec
enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

// Generic - Struct
struct Point<T> {
    x: T,
    y: T
}

// Generic - Function
fn swap<T1, T2>(a: T1, b:T2) -> (T2, T1) {
    (b, a)
}

fn main() {
    let v: Vec<u32> = vec![1u32, 2, 3];

    let p: Point<i32> = Point {
        x: 0,
        y: 0
    };

    let a: u32 = 1;
    let b: i32 = 2;
    let (a, b) = swap(a, b);
}