use std::io::{stdin, BufRead};

fn main() {
    let mut buf = String::new();
    stdin().lock().read_line(&mut buf).unwrap();

    let num_strs = buf.strip_suffix("\n").unwrap().split(" ");
    let mut sum = 0;
    for num_str in num_strs {
        match num_str.parse::<i32>() {
            Ok(num) => sum += num,
            Err(_) => {},
        }
    }
    println!("{}", sum);
}