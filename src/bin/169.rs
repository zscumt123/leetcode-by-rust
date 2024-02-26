fn main() {}

struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut result = i32::MAX;
        for val in nums {
            if count == 0 {
                result = val
            }
            count += if val == result { 1 } else { -1 }
        }
        result
    }
}
