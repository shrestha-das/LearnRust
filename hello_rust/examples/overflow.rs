#![allow(unused)]

// Overflow doesn't panic when compiled with "--release"
fn main() {
    let mut x = u32::MAX;
    x += 1;

    println!("u32 max: {}, x: {x}", u32::MAX);

    // u32::checked_add - return None on overflow otherwise Some(Ans)
    let x = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", x);

    // u32::wrapping_add - explicitly allow overflow
    let x = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {:?}", x);
}