fn main() {
    // literal match
    let x = 1;
    match x {
        1 => println!("one!"),
        2 => println!("two!"),
        3 => println!("three!"),
        _ => println!("anything!"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point {x, y}| x * x + y * y)
        .sum();
}
