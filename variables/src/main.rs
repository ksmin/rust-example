fn main() {
    let mut x: i32 = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    let guess: u32 = "42".parse().expect("Not a number!");
}
