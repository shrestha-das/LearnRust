#![allow(unused)]

use std::collections::HashMap;
// Iterator adaptore
// - map
// - filter
// - collect

fn main() {
    let vals: Vec<u32> = vec![1,2,3,4,5];

    // &u32
    // map - f(x: &u32) -> u32
    // collect
    let vals2: Vec<u32> = vals.iter().map(|x: &u32| x+1).collect();

    println!("vals2 -- {:?}", vals2);


    let vals: Vec<(&str, u32)> = vec![("a",1),("b",2),("c",3)];

    let v: Vec<(String, u32)> = vals.iter().map(|v| 
    (v.0.to_string(), v.1 +1)).collect();
    println!("vec -- {:?}", v);

    let h: HashMap<String, u32> = vals.iter().map(|v|
    (v.0.to_string(), v.1 +1)).collect();
    println!("hash map -- {:?}", h);


    // filter
    let vals: Vec<u32> = vec![1,2,3,4,5]; 
    let f: Vec<&u32> = vals.iter().filter(|x: &&u32| **x <= 3).collect();
    println!("f -- {:?}", f);

    // chaining filter and map
    let vals: Vec<u32> = vec![1,2,3,4,5]; 
    let c: Vec<u32> = vals.into_iter().filter(|x: &u32| *x <= 3).map(|m: u32| m+2).collect();
    println!("c -- {:?}", c);
}
