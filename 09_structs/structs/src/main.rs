fn main() {
    // Structs are mixed data types
    // three types of structs
    // named field, tuple struct and unit like

    let user1: User = User{
        active: true, 
        user_name: "sasha".to_string(), 
        sign_in_count: 0
    };

    // access info with dot
    println!("{}", user1.user_name);

    let usr2: User = build_user("sasha2".to_string());
    println!("{}", usr2.user_name);

    let coords: Coords = Coords(-28, 17);
    println!("{}, {}", coords.0, coords.1);

    //structs are kind of like classes, they can implement methods
    // e.g. with square
    let my_square: Square = Square{width: 2,height: 5};
    println!("{}", my_square.area());
    println!("{}", my_square.width());
    println!("{}", my_square.height());

    // mutable
    let mut sq2: Square = Square{width: 2,height: 5};
    sq2.set_width(8);
    println!("{}", sq2.width);
}

//named field struct
struct User {
    active: bool,
    user_name: String,
    sign_in_count: u32
}

//tuple struct
// identify vals by order
struct Coords(i32, i32);

// unit like struct
struct UnitStruct;

// Struct with methods
struct Square {
    width: u32,
    height: u32
}

// implement method to struct
impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }  

    fn height(&self) -> u32 {
        self.height
    }

    fn set_width(&mut self, new_width: u32) {
        self.width = new_width
    }

}

fn build_user(user_name:String) -> User{
    User{
        user_name,
        active: true,
        sign_in_count: 0
    }
}
