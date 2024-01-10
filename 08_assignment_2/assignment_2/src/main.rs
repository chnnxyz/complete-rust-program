fn main() {
    // Create a function that takes one argument called val that is of 
    // type Vec with the values: 1,3,5,7. Inside of this function check
    // the first value of the vector and see if it is equal to one. 
    // If the value is equal to one, then return true, otherwise return false. 
    // Next add the value 15 to the vector. 
    // Print out the vector to confirm your results.

    fn check_vec(v: &Vec<i32>) -> bool{
        if v[0] == 1 {
            return true
        }
        false
    }

    let mut vec: Vec<i32> = vec![1,3,5,7];
    check_vec(&vec);
    println!("{:?}", vec);
    vec.push(15);
    println!("{:?}", vec);

    // Create a function called "add_two". 
    // This function is going to take one parameter that is i8 and add two 
    // to it. 
    // For the function, do you have to pass the value by reference in order 
    // for you to maintain ownership of it inside of main?

    // i8 implements copy
    fn add_two(x:i8) -> i8 {
        x + 2
    }
    let x = 4;
    println!("{}", add_two(x));
    println!("{}", x);

}
