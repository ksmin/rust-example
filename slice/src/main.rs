fn main() {
    let mut str = String::from("Hello, world!");
    println!("{}, {:p}", str, &str);
    println!("{}", first_word(&str[..]));

    let slice = &str[0..5];
    // Same with => let slice = &str[..5];
    println!("{}, {:p}", slice, slice);

    let slice = &str[7..];
    // Same with => let slice = &str[7..];
    println!("{}, {:p}", slice, slice);

    let word = first_word(&str[..]);
    println!("{}, {:p}", word, word);
}

fn first_word(str: &str) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        println!("{}: {}", i, item.to_ascii_lowercase());
        if item == b' ' {
            return &str[0..i];
        }
    }
    &str[..]
}