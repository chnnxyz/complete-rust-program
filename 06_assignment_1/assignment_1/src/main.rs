fn main() {
    // Create three variables with the names: val1, val2, and ans. 
    // We want to perform a simple operation of generating the 
    // modulo of val1 and val2. Set val1 to 5 and val2 to 2. 
    // Assign the answer to the ans variable.

    let val1: u64 = 5;
    let val2: u64 = 2;
    let ans: u64 = val1 % val2;
    println!("{}", ans);

    // Create a vector and put in the values "2, 4, 6, 8, 10". 
    // Once you have created the vector perform the following: 
    // print out the current values, remove the value 10, add the value 12,
    // and then print the vector back out to confirm your results.

    let mut vect: Vec<u8> = vec![2,4,6,8,10];
    vect.pop();
    vect.push(12);
    println!("{:?}", vect);

    // Create a function called "concat_string". 
    // Create a string variable and assign the value "Hello" to it. 
    // The function is going to take one argument that is of type string and 
    // is going to return a String. Inside this function, 
    // concatenate the string " World". 
    // Print out the results in main() to confirm your results. 

    fn concat_string(mut s: String) -> String {
        s.push_str(" World");
        s
    }
    let x = "Hello".to_string();
    println!("{}", concat_string(x));

    // Create a function called control_flow. 
    // This is going to take one argument that is an integer. 
    // Based on this integer, print out the following: 
    // "The value is one", "The value is greater than 50", 
    // "The value is less than 25", 
    // or "The value is greater than 25 but less than 50".
    fn control_flow(x: i64) {
        if x == 1 {
            println!("The value is one")
        } else if x > 50 {
            println!("The value is greater than 50")
        } else if x < 25 {
            println!("The value is less than 25")
        } else {
            println!{"The value is greater than 25 but less than 50"}
        }
    }
}
