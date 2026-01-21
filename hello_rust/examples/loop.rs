#![allow(unused)]

fn main() {
    
    // loop
    let mut i = 0;
    loop {
        println!("loop {i}");
        i += 1;

        if i > 5 {
            break;
        }
    }

    // while loop
    let mut i = 0;
    while i <= 5 {
        println!("while loop {i}");
        i += 1;
    }

    // for loop
    for i in 0..6 {
        println!("for loop {i}");
    }

    // array
    let arr = [0,1,2,3,4,5];
    let n: usize = arr.len();
    println!("n = {n}");
    for i in 0..n {
        println!("n = {n}");
        println!("1)array loop {}", arr[i]);
    }

    for n in arr {
        println!("n = {n}");
        println!("2)array loop {}", n);
    }

    // vector
    let v = vec![10,20,30,40,50,60];

    for n in &v {
        println!("vector {}", n);
    }
    
    for n in v.iter() {
        println!("vector {}", n);
    }

    for n in v.into_iter() {
        println!("vector {}", n);
    }

    // return a value from a loop
    let mut i = 0;
    let z: &str = loop {
        println!("loop {i}");
        i += 1;

        if i > 5 {
            break "loop ends here";
        }
    };
    println!("loop returns: {z}");
}