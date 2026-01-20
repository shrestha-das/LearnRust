#![allow(unused)]

use std::collections::HashMap;

// HashMap
fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    println!("{:?}", scores);
    scores.insert("red".to_string(), 100);
    scores.insert("blue".to_string(), 200);
    println!("{:#?}", scores);

    // Get 
    println!("Red Score: {:?}", scores.get("red"));
    // returns Some(T) or None

    // Update
    let score: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *score += 100;

    let score: Option<&u32> = scores.get("blue");
    println!("Blue score: {:?}", score);

    
    let score: &mut u32 = scores.entry("black".to_string()).or_insert(0);
    *score += 100;

    let score: Option<&u32> = scores.get("black");
    println!("Black score: {:?}", score);
}