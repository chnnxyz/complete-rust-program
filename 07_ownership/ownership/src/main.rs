fn main() {
    // what is ownership
    // heap requests space and returns a pointer
    // searching stack is faster
    //  data in a function is in the stack

    // Rules of ownership
    // Each variable has an owner,
    // One owner at a time
    // Out of scope means kill from heap.

    // This is in stack
    let var: u32 = 1;
    // This is crested on the heap
    // mutables are always in the heap
    let mut str: String = "Hello".to_string();
    str.push_str(", world");

    // Move: Move ownership from one var to other
    let x: Vec<String> = vec!["uwu".to_string()];
    // This switches x ownership into y
    let y = x;
    // therefore this line breaks
    // println!("{:?}", x)
    // This works tho
    let z = y;
    println!("{:?}",z);

    // Clone: expensive in memory, makes a deepcopy
    let z2 = z.clone();
    println!("{:?}", z);
    println!("{:?}", z2);

    // Copy: shallow copy
    // Copy is by default implemented in vars in the stack

    // More moves
    let s: String = String::from("uwu");
    take_ownership(s); // Give ownership to fun
    // This would break as ownership was given to take_ownership
    // and then killed it
    // println!("{}", s)
    let s2: String = give_ownership();
    println!("{}", s2);

    // Control flow statements also take ownership.

    // References and borrowing
    // References borrow the value, immutable refs are infinite
    // mutable refs can only happen once because lifetimes or smth

    let mut s: String = "helo".to_string();
    change_str(&mut s);
    println!("{}", s);

}

fn change_str(str: &mut String) {
    str.push_str(", world")
}

fn take_ownership(s: String) {
    let str = s;
    println!("{}", str)
}

fn give_ownership() -> String {
    "given".to_string()
}
// Here var is dropped
