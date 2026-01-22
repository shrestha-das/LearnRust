#![allow(unused)]

// Ownership rules

fn take(s: String) {
    println!("take {s}");
    // s is dropped
    println!("{s}"); 
}

fn copy(v: i32) {
    println!("copy {v}");
    // v is dropped here
}

fn main() {
    // 1. Each value has an owner

    // Owner of s is s
    let s = String::from("rust");
    // Owner of i is i
    let i = 1;

    // 2. There can only be one owner at a time

    // Owner of s is s
    let s = String::from("rust");
    // Now, owner of s is s1
    let s1 = s;
    // Now, owner of s is s2;
    let s2 = s1;
    println!("{s2}");
    // println!("{s1}"); // It will not compile cuz it's not the owner anymore
    // println!("{s}"); // It will not compile as well

    // 3. When the owner goes out of scope, the value will be dropped
    let s = String::from("cat");
    {
        s;
        // s is dropped
    }   // scope ends here
    // println!("{s}"); // will not compile
    
    let s = String::from("cat");
    {
        // Owner of "cat" is s1
        let s1 = s;
        // s1 is dropped
        // println!("{s}"); // will not compile
    }   // scope ends here
    // println!("{s}"); // will not compile

    let s = String::from("cat");
    take(s);
    // println!("{s}"); // will not compile

    // Ownership doesn't move for types that implement the Copy traits
    // Owner of i is 1
    let i = 1;
    // Owner of i1 is i1
    let i1 = i;
    // Owner of i2 is i2
    let i2 = i1;

    copy(i);
    println!("{i}");
}