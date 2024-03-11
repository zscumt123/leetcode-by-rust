fn main() {
    let nums = vec![1, 2, 3];
    let result = Solution::subarray_sum(nums, 3);
    println!("result,{}", result)
}

use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cache = HashMap::from([(0, 1)]);
        let (mut pre, mut result) = (0, 0);
        for val in nums {
            pre += val;
            if let Some(t) = cache.get(&(pre - k)) {
                result += *t;
            }

            *cache.entry(pre).or_insert(0) += 1;
        }

        result
    }
}
