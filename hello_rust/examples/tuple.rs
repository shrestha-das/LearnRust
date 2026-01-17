#![allow(unused)]

fn return_many() -> (u32, bool) {
    (1u32, true)
}

// Tuples - fixed size, mixed types, known at compile time
fn main() {
    let t: (bool, char, u32) = (true, 'a', 1);
    println!("{}, {}, {}", t.0, t.1, t.2);

    // Empty tuple = unit type
    let t = ();

    // Nested tuple
    let nested_t = (('a', 1.23), (true, 1u32, -1i32,), ());
    println!("nested_t.0.1 : {}", (nested_t.0).1 );

    // Destructuring a tuple
    let (a,b) = nested_t.0;
    println!("a: {}, b : {}", a, b );

    // Partial destructuring 
    let (_,b) = nested_t.0;

    // Function that return multiple values using a tuple
    let (x, y) = return_many();
    println!("x : {}, y : {}", x, y);
}