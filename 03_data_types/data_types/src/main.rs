use core::slice;

fn main() {

    // Compiler will infer types.

    // Integers
    // can be signed (i) or unsigned (positive only) (u)
    // can be 8, 16, 32, 64 or 128 bit
    // e.g. u8, 8 bit unsigned
    // usize and isize use platform size

    // 10 is a perfectly valid 8 bit int
    let x: i8 = 10;
    println!("{}", x);

    let y: u8 = 10;
    println!("{}", y);

    // Integer literals
    // can be decimal, hex, octal, binary
    // underscores only help for readability
    // All these are 255, allowed in u8 but not i8
    let decimal: u8 = 02_55;
    let hex: u8 = 0xff;
    let octal: u8 = 0o377;
    let binary: u8 = 0b11111111;
    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let byte: u8 = b'A';
    println!("The ascii value of A is {}", byte);

    // Floats are similar to ints, but with a floating point.
    // Precisions are the same, and floats are always signed.

    let _x:f64 = 2.0;
    let _y:f32 = 2.0;

    // booleans: true/false 1/0.
    let _t: bool = true;
    let _f: bool = false;

    // characters: single character
    let c: char = 'c';
    println!("{}", c);

    // Arithmetics:
    // +, -, * , / basic ops
    // % modulo
    // powers are done with base.pow

    // Compound types: Tuples and arrays
    // Tuples have fixed length and can take any types.
    let tup: (i32, char, bool) = (1, 'a', true);
    // tuples are accesed with dot
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    // As in elixir and python, there is pattern matching
    let (_x, _y, _z) = tup;

    // Arrays have the same type for all elements
    // Type hinting is type; length
    let arr: [u8; 3] = [1,2,3];
    // To access elements we use brackets
    println!("{}", arr[0]);
    // both tuples and arrays are mutable using let mut

    // Vectors:
    // Resizable arrays in heap
    // created with the vec! macro
    // type is Vac<type>
    // vectors cannot change if no let mut
    //  vecs are essentially single-typed python lists.
    let mut nums = vec![1,2,3];
    nums.push(4);
    println!("{:?}", nums);
    //alternatively, you can use
    let mut vec = Vec::new();
    vec.push("test");
    vec.push("string");
    // this infers a Vec<&str> type.
    println!("{:?}", vec);

    //you can predefine vec size
    let mut vect = Vec::<i32>::with_capacity(2);
    println!("{}", vect.capacity());
    // resizing essentially copies to a new buffer and updates pointer

    // here (0..5 is an iterator)
    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    // Slice: region of array or Vec. Cannot be stored to vars or passed as
    // function args.
    // Here we are not assigning the variable, but the pointer to the vec mem
    // address
    let slice : &[i32] = &v;
    println!("{:?}", slice);

    // A slice can also point to specific elements
    // it has 2 included but 4 not included
    let slice: &[i32] = &v[2..4];
    println!("{:?}", slice);

    // Strings and &str
    // Strings are always valid utf8
    let name: String = String::from("uwu");
    //alternatively
    let course: String = "Rust".to_string();
    let new_name: String = name.replace("uwu", "owo");
    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    // &str, on the other hand is a string slice
    // As with vectors, it is just a pointer to parts of a string
    // &str is not mutable
    let str1: &str = "str";
    println!("{}", str1);

    // String slices can refer to strings or string literals.
    // String slices dont allocate memory
    // the previous to_string method converts &str into String
    // referencing the string with & makes it into &str

    // String Literals
    // String and &str are valid utf8
    // String Literals are good for non utf-8
    // they are essentially &str for non-valid utf8
    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);

}
