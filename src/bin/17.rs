use std::collections::HashMap;

struct Solution;
use std::collections::VecDeque;
use std::fmt::format;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mut res = VecDeque::new();
        res.push_back("".to_string());

        let cache = [
            vec![],
            vec![],
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];

        for char in digits.chars() {
            let index = char.to_digit(10).unwrap() as usize;
            let len = res.len();
            for i in 0..len {
                if let Some(current) = res.pop_front() {
                    let list = &cache[index];
                    list.into_iter().for_each(|v| {
                        let c = current.clone() + &v.to_string();
                        res.push_back(c);
                    })
                }
            }
        }

        res.into_iter().collect::<Vec<_>>()
    }
}
