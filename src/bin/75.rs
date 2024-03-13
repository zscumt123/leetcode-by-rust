use std::mem::swap;

fn main() {}
struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut p0, mut p1) = (0, 0);
        for i in 0..nums.len() {
            if nums[i] == 1 {
                nums.swap(i, p1);
                p1 += 1;
            } else if nums[i] == 0 {
                nums.swap(i, p0);

                if (p0 < p1) {
                    nums.swap(i, p1)
                }
                p0 += 1;
                p1 += 1;
            }
        }
    }
}
