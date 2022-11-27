fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    // v3.push("3");

    for value in &mut v3 {
        println!("before: {}", value);
        *value *= 2;
        println!("after: {}", value);
    }

    for value in &v3 {
        println!("changed: {}", value);
    }

    let e1 = &v2[1];
    // let e1 = &v3[3];
    println!("e1: {}", e1);

    let e2 = v2.get(3);
    println!("e2: {:?}", e2);

    v2.push(4);
}
