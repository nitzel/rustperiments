#[path = "ch5_user.rs"]
mod user;

pub fn main() {
    ch5_1();
    ch5_2and3();
}

// Tuple Structs
struct Color(u32, u32, u32);
struct Point3D(i32, i32, i32);

/// Chapter 5.1
fn ch5_1() {
    // Chapter 5.1
    let user = user::create2("John Doe", "doe@john.org");
    let mut user2 = user::clone(&user);
    user2.email = String::from("other@mail.org");

    println!("User1: {}", user::to_string(&user));

    println!("User1: {}", user::to_string(&user2));

    // Tuple Structs
    let orange = Color(0xfc, 0x9a, 0x04);
    let origin = Point3D(0, 0, 0);
    println!("Color orange: r{} g{} b{}", orange.0, orange.1, orange.2);
    println!("Point origin: x{} y{} z{}", origin.0, origin.1, origin.2);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn longest_side(&self) -> u32 {
        if self.width > self.height {
            self.width
        } else {
            self.height
        }
    }

    fn transpose(&self) -> Rectangle {
        Rectangle {
            width: self.height,
            height: self.width,
        }
    }

    fn quadruple(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(width: u32) -> Rectangle {
        Rectangle {
            width,
            height: width,
        }
    }
}

/// Chapter 5.2 and Chapter 5.3
fn ch5_2and3() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("area of {:?} is {}", rect, rect.area());
    println!("longest side of {:?} is {}", rect, rect.longest_side());
    let mut rect = rect.transpose();
    println!("longest side of {:?} is {}", rect, rect.longest_side());
    rect.quadruple();
    println!("longest side of {:?} is {}", rect, rect.longest_side());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "Can rect3 hold rect2? {}",
        Rectangle::can_hold(&rect3, &rect2)
    );
    println!("Creating a square: {:?}", Rectangle::square(33));
}
