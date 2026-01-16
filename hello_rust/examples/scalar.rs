#![allow(unused)]

// Scalar - data types that represent a single value
fn main() {
    // Signed integers
    // Unsigned integers 
    // Foating point numbers
    // ** Depends on computer architecture
    let i5: isize = -6;
    let u5: usize = 6;
    // Boolean
    // Characters
    let c: char = 'c'; // single-quote('') for char
    let r = '🦀';

    // ** Type conversion
    let i: i32 = -1;
    let u: u32 = i as u32;
    println!("{i} as u32 is {u}");

    // Min and Max
    let i_max = i32::MAX;
    let u_min = u32::MIN;
    println!("i max: {i_max}");
    println!("u min: {u_min}");
} 