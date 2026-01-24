#![allow(unused)]

// unwrap
// expect

fn main() {
    let x: Option<i32> = Some(3);
    let v: i32 = match x {
        Some(val) => val,
        None => panic!("no value")
    };
    // let x: Option<i32> = None; // panicked
    // Unwraps the inner value. Panics if None
    let i = x.unwrap();
    println!("Option value {}", i);


    let x: Result<i32, String> = Ok(3);
    let v: i32 = match x {
        Ok(val) => val,
        Err(err) => panic!("err: {:?}", err)
    };

    let x: Result<i32, String> = Err("error".to_string()); 
    x.expect("something failed!");
    // let i = x.unwrap();
    // println!("result: {}", i)
}