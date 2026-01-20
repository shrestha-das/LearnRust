#![allow(unused)]

// Struct 
#[derive(Debug)]
struct Point{
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Point3D (i32, i32, i32);

#[derive(Debug)]
struct Empty;

// Nested Struct
#[derive(Debug)]
struct Circle {
    radius: u32,
    center: Point
}

fn main() {
    let p = Point {x: 1, y: 1};
    println!("{:?}", p);
    println!("x: {}, y: {}", p.x, p.y);

    let p3 = Point3D(1, -1, 9);
    println!("{ :?}", p3);
    println!("x: {}, y: {}, y:{}", p3.0, p3.1, p3.2);

    let empty = Empty;
    println!("empty struct: {:?}", empty);

    let circle = Circle {
        radius: 2,
        center: Point {x: 0, y:0}
    };
    println!("{:#?}", circle);

    // Operations on struct--
    let x: i32 = 1;
    let y: i32 = 1;
    let p = Point {x: x, y: y};
    // Shortcut
    let p = Point {x, y};

    // Copy fields
    let p0 = Point {x: 1, y: 2};
    let p1 = Point {x: 2, y: p0.y};
    // or
    let p1 = Point {x: 2, ..p0};
    println!("p1 copy: {:?}", p1);

    // Update
    let mut p = Point {x: 1, y: 1};
    p.x += 1;
    p.y = 99;
    println!("updated point: {:?}", p);
}