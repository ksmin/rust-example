fn main() {
    let mut user1 = build_user(
        String::from("Sangmin, Kang"),
        String::from("ksmini82@hotmail.com"),
    );
    println!("{} <{}>", user1.name, user1.email);

    user1.email = String::from("ksmini82@gmail.com");
    println!("{} <{}>", user1.name, user1.email);

    let user2 = User {
        name: String::from("ksmin"),
        email: String::from("ksmin@saram.app"),
        ..user1
    };
    println!("{} <{}>", user2.name, user2.email);

    let black = RGBColor(0, 0, 0);
    println!("black: {}, {}, {}", black.0, black.1, black.2);

    let origin = Point3D(0, 0, 0);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);

    let rect = Rectangle {
        width: 70,
        height: 200,
    };
    println!("rect: {:#?}, {}", rect, rect.area());

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 fold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 fold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(120);
    println!("square: {:#?}", square);
}

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct RGBColor(i32, i32, i32);
struct Point3D(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}