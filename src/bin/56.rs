use std::cmp::Ordering;

fn main() {
    let res = Solution::merge(vec![vec![1, 4], vec![4, 5]]);

    println!("res:{:?}", res)
}

pub struct Solution;
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {}
}
