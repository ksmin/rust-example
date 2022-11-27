fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);

    let msg = Message::Write(String::from("hello"));
    msg.call();

    // Option, some, None
    let some_number: Option<i32> = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // let sum = some_number + absent_number;

    // match
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quater(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // if let
    let some_u8_value = Some(3);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_type: IpAddrKind) {
    println!("Hello, IP{:?}!", ip_type);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?} called!", self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None => None,    // Not cover all cases
        Some(i) => Some(i + 1),
        _ => None,
    }
}