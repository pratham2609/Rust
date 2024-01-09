// first parameter is always self, which represents the instance of the struct the method is being called on.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// everything in the implementation block will be related to Rectangle struct

// The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for.
// Methods must have a parameter named self of type Self for their first parameter,
// so Rust lets you abbreviate this with only the name self in the first parameter spot.
// Note that we still need to use the & in front of the self shorthand to indicate that this method borrows the Self instance,
// just as we did in rectangle: &Rectangle. Methods can take ownership of self, borrow self immutably, as we’ve done here,
// or borrow self mutably, just as they can any other parameter.
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }


impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq =  Rectangle::square(40);

    println!("{:#?}", sq);

}