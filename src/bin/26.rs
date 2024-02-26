fn main() {}

struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut current = i32::MAX;
        let mut index = 0;
        for i in 0..nums.len() {
            if nums[i] != current {
                current = nums[i];
                nums[index] = current;
                index += 1;
            }
        }
        index as i32
    }
}
