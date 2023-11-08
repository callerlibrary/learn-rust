use std::num::Wrapping;

fn main() {
    for_test();
    let x: u8 = 255;
    println!("The value of x is: {x}");

    let y: Wrapping<u8> = Wrapping(255);
    let y: Wrapping<u8> = y + Wrapping(1);
    println!("The value of x is: {y}");

    let z: u8 = x.wrapping_add(1);
    println!("The value of z is: {z}");

    let c: char = 'c';
    let cv: &str = "cv";
    println!("The value of c and cv is: {c} {cv}");

    let g = s();
    println!("The value of g is: {g}");
}

fn s() -> u8 {
    return 1;
}

fn for_test() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
