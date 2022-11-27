fn main() {
    let number = 4;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Wrong! if number { println!("number was something other than zero"); }
    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = false;
    // Wrong! let number = if condition { 5 } else { "six" };
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LISTOFF!!!");

    let a = [10, 20, 30, 40, 50, 60];
    let mut index = 0;

    while index < 6 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in a.iter().rev() {
        println!("the value is: {}", number);
    }

    for number in (1..=5).rev() {
        println!("{}!", number);
    }
    println!("LISTOFF!!!");
}
