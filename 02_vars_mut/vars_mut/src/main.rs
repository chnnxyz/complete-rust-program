// All variables are immutable by default
// Rust is statically typed

fn main() {
    // x is an integer
    let x = 5;
    println!("value of x: {}", x);
    // this line fails
    // x = 6;
    // since x was not defined with let mut
    let mut y: i32 = 5;
    println!("value of y: {}", y);
    y = 6;
    println!("new value of y: {}", y);
    // const, contrary to let, do not die if not passed by reference
    // they live for the full program's lifetime
    const Z: f32 = 9.81;
    println!("Gravity is {}", Z);
}
