// use std::io::{stdin, BufRead};

// fn get_line(buf: &mut String) {
//     stdin().lock().read_line(buf).unwrap();
// }

// fn parse_i32(num_str: &str) -> i32 {
//     num_str.trim().parse::<i32>().unwrap()
// }

// fn parse_numbers(numbers_str: &String) -> Vec<i32> {
//     let mut numbers = Vec::new();
//     for num_str in numbers_str.split(" ") {
//         numbers.push(parse_i32(num_str));
//     }
//     numbers
// }

// fn main() {
//     let mut num_str = String::new();
//     get_line(&mut num_str);
//     let num_count= parse_i32(&num_str);

//     let mut numbers_str = String::new();
//     get_line(&mut numbers_str);
//     let numbers = parse_numbers(&numbers_str);

//     let mut target_num = String::new();
//     get_line(&mut target_num);
//     let target_num = parse_i32(&target_num);

//     let mut count = 0;
//     for number in numbers {
//         if number == target_num {
//             count += 1;
//         }
//     }
//     println!("{}", count);
// }


fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for elem1 in nums.iter().enumerate() {
        for elem2 in nums[(elem1.0 + 1)..].iter().enumerate() {
            if elem1.1 + elem2.1 == target {
                return vec![
                    elem1.0 as i32,
                    (elem1.0 + 1 + elem2.0) as i32,
                ];
            }
        }
    }
    vec![]
}


fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("{:?}", result);
}