fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = five();

    let y = {
        let z = 3;
        plus_one(z + 1)
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    another_function(x, y);
}

fn another_function(x: i32, y: i32) {
    println!("The value is {}", x);
    println!("The value is {}", y);
}
