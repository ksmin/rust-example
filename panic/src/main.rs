use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;

fn main() {
    // do_painc();
    // do_result1();
    // do_result2();
    let result = read_username_from_file();
    println!("{:#?}", result);
    let result = read_username_from_file2();
    println!("{:#?}", result);

    use std::net::IpAddr;
    let home = "127.0.0.1".parse::<IpAddr>();
    println!("{:#?}", home);
}

fn do_result1() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Tried to create file but there was a problem: {:?}", err),
            }
        }
        Err(error) => panic!("There was a problem opening the file: {:#?}", error),
    };
}

fn do_result2() {
    let f = File::open("world.txt").expect("Failed to open world.txt");
    let f = File::open("world.txt").unwrap();
}

fn do_painc() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    v[99];
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    // let mut f = File::open("hello.txt")?;
    // f.read_to_string(&mut s)?;

    File::open("hello.txt")?
        .read_to_string(&mut s)?;

    Ok(s)
}