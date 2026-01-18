#![allow(unused)]

// Array - collection of elements with length known at compile time
// Slice - collection of elements with length not known at compile time
fn main() {
    // Array 
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[0]: {}", arr[0]);
    println!("arr: {:?}", arr);

    // Write 
    let mut arr: [u32; 3] = [1, 2, 3];
    arr[1] = 99;
    println!("arr: {:?}", arr);

    let arr: [u32; 10] = [0; 10];
    println!("arr: {:?}", arr);


    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];
    // Slice
    let s: &[i32] = &nums[..3];
    println!("First 3 elements of nums array are {:?}", s);
    let s: &[i32] = &nums[7..];
    println!("Last 3 elements of nums array are {:?}", s);
    let s: &[i32] = &nums[3..7];
    println!("Middle 4 elements of nums array are: {:?}", s);
}