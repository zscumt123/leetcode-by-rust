fn main() {}
use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut cache = HashSet::new();
        for val in nums.iter() {
            cache.insert(*val);
        }

        let mut result = 0;

        for val in nums {
            if !cache.contains(&(val - 1)) {
                let (mut i, mut len) = (val, 0);

                while cache.contains(&i) {
                    i += 1;
                    len += 1;
                }
                result = result.max(len)
            }
        }

        result
    }
}
