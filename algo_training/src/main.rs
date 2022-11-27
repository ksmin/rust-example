// use std::io::{stdin, BufRead};

// fn main() {
//     let mut buf = String::new();
//     stdin().lock().read_line(&mut buf).unwrap();

//     let num_strs = buf.strip_suffix("\n").unwrap().split(" ");
//     let mut sum = 0;
//     for num_str in num_strs {
//         match num_str.parse::<i32>() {
//             Ok(num) => sum += num,
//             Err(_) => {},
//         }
//     }
//     println!("{}", sum);
// }



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


// fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     for elem1 in nums.iter().enumerate() {
//         for elem2 in nums[(elem1.0 + 1)..].iter().enumerate() {
//             if elem1.1 + elem2.1 == target {
//                 return vec![
//                     elem1.0 as i32,
//                     (elem1.0 + 1 + elem2.0) as i32,
//                 ];
//             }
//         }
//     }
//     vec![]
// }


// fn main() {
//     let nums = vec![2, 7, 11, 15];
//     let target = 9;
//     let result = two_sum(nums, target);
//     println!("{:?}", result);
// }

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn convert_list_to_number(listnode: &Option<Box<ListNode>>) -> u128 {
    let mut number: u128 = 0;
    let mut operand: u128 = 1;
    let mut node_box = listnode;
    while *node_box != None {
        let node = match node_box {
            Some(node_box) => node_box,
            None => break,
        };
        number += node.val as u128 * operand;
        operand *= 10;
        node_box = &node.next;
    }
    number
}

// fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let num1 = convert_list_to_number(&l1);
//     let num2 = convert_list_to_number(&l2);
//     convert_number_to_list(num1 + num2)
// }

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut box1 = &l1;
    let mut box2 = &l2;
    let mut carry = 0;
    let mut head = None;
    let mut cursor = &mut head;

    while *box1 != None || *box2 != None {
        let mut sum = carry;
        sum += match box1 {
            Some(node) => {
                let val = node.val;
                box1 = &node.next;
                val
            },
            None => 0,
        };
        sum += match box2 {
            Some(node) => {
                let val = node.val;
                box2 = &node.next;
                val
            },
            None => 0,
        };
        carry = sum / 10;

        *cursor = Some(Box::new(ListNode::new(sum % 10)));
        cursor = &mut cursor.as_mut().unwrap().next;
    }
    if carry > 0 {
        *cursor = Some(Box::new(ListNode::new(carry)));
    }
    head
}

fn convert_number_to_list(number: u128) -> Option<Box<ListNode>> {
    let num_str = number.to_string();
    let mut cur_node: Option<Box<ListNode>> = None;
    for ch in num_str.chars() {
        let mut new_node = ListNode::new(ch.to_digit(10).unwrap() as i32);
        if cur_node != None {
            new_node.next = cur_node;
        }
        cur_node = Some(Box::new(new_node));
    }
    cur_node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_number_to_list() {
        let l1 = convert_number_to_list(342);
        let first_node = l1.unwrap();
        assert_eq!(2, first_node.val);
        let second_node = first_node.next.unwrap();
        assert_eq!(4, second_node.val);
        let third_node = second_node.next.unwrap();
        assert_eq!(3, third_node.val);
    }

    #[test]
    fn test_convert_list_to_number() {
        let l1 = convert_number_to_list(342);
        let number = convert_list_to_number(&l1);
        assert_eq!(number, 342);
    }

    #[test]
    fn test_add_two_numbers_1() {
        let l1 = convert_number_to_list(342);
        let l2 = convert_number_to_list(465);
        let result = add_two_numbers(l1, l2);

        let first_node = result.unwrap();
        assert_eq!(7, first_node.val);
        let second_node = first_node.next.unwrap();
        assert_eq!(0, second_node.val);
        let third_node = second_node.next.unwrap();
        assert_eq!(8, third_node.val);
    }

    #[test]
    fn test_add_two_numbers_2() {
        let l1 = convert_number_to_list(9);
        let l2 = convert_number_to_list(9999999991);
        let result = add_two_numbers(l1, l2);

        let node = result.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(1, node.val);
    }

    #[test]
    fn test_add_two_numbers_3() {
        let l1 = convert_number_to_list(1000000000000000000000000000001);
        let l2 = convert_number_to_list(564);
        let result = add_two_numbers(l1, l2);

        let node = result.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(0, node.val);
        let node = node.next.unwrap();
        assert_eq!(1, node.val);
    }
}