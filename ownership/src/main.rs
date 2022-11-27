fn main() {
    let hello_str: &str = "Hello";
    println!("{}, world!", hello_str);

    let mut hello_string: String = String::from(hello_str);
    hello_string.push_str(", world!");
    println!("{}", hello_string);

    let hello_string2: String = hello_string;
    println!("{}", hello_string2);
    // Moved! println!("{}", hello_string);

    let hello_clone: String = hello_string2.clone();
    println!("{}({:p}) = {}({:p})", hello_string2, hello_string2.as_ptr(), hello_clone, hello_clone.as_ptr());

    takes_ownership(hello_string2);
    // Moved! println!("{}", hello_string2);

    let x: i32 = 5;
    makes_copy(x);

    let hello_string3: String = takes_and_gives_back(hello_clone);
    // Moved! println!("{}", hello_clone);
    println!("{}", hello_string3);

    let (length, hello_string4) = calculate_length(hello_string3);
    println!("The length of '{}' is {}.", hello_string4, length);

    let len = calc_length_ref(&hello_string4);
    println!("The length of '{}' is {}.", hello_string4, len);  // Not moved!

    let mut hello_string5: String = hello_string4.clone();
    println!("{}(not changed yet)", hello_string5);
    change_ref(&mut hello_string5);
    println!("{}(changed!!!)", hello_string5);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(str: String) -> (usize, String) {
    (str.len(), str)
}

fn calc_length_ref(str: &String) -> usize {
    str.len()
}

fn change_ref(str: &mut String) {
    str.push_str("string!!");
}