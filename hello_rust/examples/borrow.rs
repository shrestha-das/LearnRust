#![allow(unused)]

fn take(str: String) {
    //println!("take {s}");
}

fn borrow(s: &String) {
    println!("borrow {s}");
}

// Borrow - temporarily use a value without taking ownership

fn main() {
    // Take ownership
    let s = String::from("rust");
    //take(s);
    borrow(&s);
    // s is dropped after take(s)
    // This will not compile
    //println!("{s}");

    // - Creates a reference (either mutable or immutable)
    // - - immutable borrow
    let s = String::from("rust");
    // s1, s2 and s3 have read-only access to s
    let s1 = &s;
    let s2 = &s;
    let s3 = &s;
    // - - mutable borrow
    let mut s = String::from("rust");
    // s1 has read and write access to s
    let s1 = &mut s;
    s1.push_str("🦀");
    let s2 = &mut s;
    s2.push_str("🦀");

    // - Either immutable or mutable borrow, but not both simultaneously.
    let mut s = String::from("rust");
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s;
    println!("s1: {s1}");
    // s3.push_str("CRABBBBBB!!!!!!");

    // - Reference must not outlive the value( 1st way)
    // let mut s = String::from("rust");
    // let s1 = &s;
    // {
    //     let s2 = s;
    // }
    // // s2 and s - no longer exist
    // // s1 references s
    // println!("s1: {s1}");

    // - Doesn't move ownership
    // - Immutable reference - any number of read-only access to a value
    // - Mutable reference - only one read and write access to a value at time
}

// - Reference must not outlive the value( 2nd way)
// fn dangle(s: String) -> &String {
//     &s
// }