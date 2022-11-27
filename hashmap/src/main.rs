fn main() {
    use std::collections::HashMap;

    // let mut scores = HashMap::new();
    // scores.insert("Blue".to_string(), 10);
    // scores.insert("Yellow".to_string(), 50);
    // println!("Hello, world!");

    // zip
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    println!("{:?}", scores.get(&String::from("Blue")));

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    scores.entry(&String::from("Blue")).or_insert(&100);
    scores.entry(&String::from("White")).or_insert(&100);

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
}
