// Define functions with fn

// No args, no returns
fn print_phrase() {
    println!("Hello from print_phase");
}

// With params, without returns
fn print_phrase_2(phrase: &str) {
    println!("{}", phrase);
}

// With params, with returns
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    // last line without semicolon returns
    // or use the return keyword
    b
}

// Multiple returns
fn mrv(flag:bool) -> bool {
    if flag {
        return true
    }
    false
} 

fn main() {
   print_phrase();
   print_phrase_2("uwu");
   let z = gcd(10, 30);
   println!("gcd is {}", z);
   println!("{}", mrv(true));
   println!("{}", mrv(false))
}
