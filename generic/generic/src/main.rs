struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    let p = Point { x: 1.0, y: 4.0 };
    println!("p.x = {}", p.x());


    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {result}");

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("The largest character is {result}");
}

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest_item = list[0];
//     for &item in list {
//         if item > largest_item {
//             largest_item = item;
//         }
//     }
//     largest_item
// }

fn largest<T>(list: &[T]) -> &T {
    let mut largest_item = list[0];
    for &item in list {
        if item > largest_item {
            largest_item = item;
        }
    }
    &largest_item
}