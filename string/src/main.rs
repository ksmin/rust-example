fn main() {
    let s = String::new();
    println!("1: {}", s);
    let data = "initial contents";
    println!("2: {}", data);
    let s = data.to_string();
    println!("3: {}", s);
    let s = "initial contents".to_string();
    println!("4: {}", s);

    let mut s = String::from("Hello");
    println!("5: {}", s);
    s.push_str(", world");
    println!("6: {}", s);

    let mut s1 = String::from("foo");
    println!("7: {}", s1);
    let s2 = "bar";
    s1.push_str(&s2);
    println!("8: {}", s1);
    println!("9: {}", s2);

    let mut s = String::from("lo");
    println!("10: {}", s);
    s.push('l');
    println!("11: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("12: {}", s3);
    // println!("13: {}", s1); Moved!
    println!("14: {}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("15: {}", s);
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("16: {}", s);

    let kor_s1 = "안녕하세요";
    println!("17: {}, {}", kor_s1, kor_s1.len());
    println!("18: {}", &kor_s1[0..3]);
    for c in kor_s1.chars() {
        println!("19: {}", c);
    }
    for b in kor_s1.bytes() {
        println!("20: {}", b);
    }
}
