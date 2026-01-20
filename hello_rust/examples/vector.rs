#![allow(unused)]

// Vector
// Vectors are like Arrays, a collection of elements having the same datatype
// The key difference between them--
// Arrays' fixed size known at the compile time
// Vector's size can grow or shrink accordingly at runtime

fn main() {
    let mut v: Vec<i32> = Vec::new(); // new empty vector
    println!("new v: {:?}", v);
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);

    let v = vec![1u8,2,3]; // vector already containing elements
    println!("v with elements: {:?}", v);

    // vector that has 100 elements with 0 value each
    let v: Vec<i8> = vec![0i8; 100];
    println!("v: {:?}", v);

    // Get an element of Vector
    println!("v[1]: {:?}", v[1]);
    // println!("v[100]: {:?}", v[100]);
    // In the above case terminal panicks

    // instead use .get() to handle this kinda panicking situation
    println!("v[1]: {:?}", v.get(1));
    println!("v[100]: {:?}", v.get(100));
    // Option<&i8>
    // valid index => Some(&val)
    // invalid index => None

    // Update
    let mut v: Vec<i8>  = vec![1,5,4];
    v[0] = 99;
    println!("updated v: {:?}", v);

    // Pop = remove the last item
    let x: Option<i8> = v.pop();
    println!("popped element: {:?}", x);
    println!("updated v: {:?}", v); 

    // Slice from Vector 
    v.push(1);
    v.push(3);
    println!("v: {:?}", v);
    let s = &v[1..3];
    println!("slice of v: {:?}", s);
}