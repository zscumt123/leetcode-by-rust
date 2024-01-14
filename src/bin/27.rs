fn main() {
    let mut res = vec![3, 2, 2, 3];
    Solution::remove_element(&mut res, 3);
}
struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut index, len) = (0, nums.len());
        for i in 0..len {
            if nums[i] == val {
                nums[index] = nums[i];
                index += 1;
            }
        }
        index as i32
    }
}
