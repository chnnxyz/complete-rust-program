fn main() {
    // if
    let one: u8 = 1;
    if one > 1 {
        println!("Greater than one")
    } else if one == 1 {
        println!("Exactly one")
    } else {
        println!("Zero")
    }

    // loop
    // cycles infinitely until broken
    // loop {
    //     println!("loop")
    // }

    let mut num = 0;
    // this is a lifetime but course not saying it yet
    'counter: loop {
        let mut dec = 5;
        println!("Count: {}", num);
        loop {
            println!("decreasing {}", dec);
            if dec == 4 {
                break
            }
            if num == 2 {
                // You can break outer loops
                break 'counter
            }
            dec -= 1;
        }
        num += 1;
    }
    // Continue keyword also exists

    // while
    let mut num = 0;
    while num < 5 {
        println!("{}", num);
        num += 1;
    }

    // for
    let vec: Vec<u8> = (0..10).collect();
    for el in vec {
        println!("{}", el)
    }

    for n in (1..4).rev() {
        println!("{}", n)
    }
    println!("liftoff")

}
