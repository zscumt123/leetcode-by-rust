fn main() {}

struct Solution;
impl Solution {
    //nums = [0,1,0,3,12]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut i, len) = (0, nums.len());

        for j in 0..len {
            if nums[j] != 0 {
                nums.swap(i, j);
                i += 1;
            }
        }
    }
}
