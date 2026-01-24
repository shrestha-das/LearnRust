#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivByZero,
    Other
}

fn main() {
    // Error handling
    // Usual way :- panic!("Something went wrong!");

    let v = vec![1,2,3];
    // Index out of bounds
    // v[99];

    // 1st way :- Option type (Some(val) or None)
    let x: Option<&i32> = v.get(1);
    match x {
        Some(val) => println!("x: {:?}", val),
        None => println!("x: none")
    }

    // 2nd way :- Result type (Ok(T) or Err(E))
    let x = 1;
    let y = 0;

    // This will panic. Division by 0
    // let q = x / y;
    // So use this method -
    let q: Result<i32, MathError> = if y != 0 {
        Ok(x/y)
    } else {
        //Err("div by 0".to_string())
        Err(MathError::DivByZero)
    };
    match q {
        Ok(val) => println!("x/y = {:?}", val),
        Err(err) => println!("x/y error {:?}", err)
    }
}