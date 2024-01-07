use core::num;

fn main() {}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let (mut left, mut sum, mut result) = (0, 0, i32::MAX);

    for (right, v) in nums.iter().enumerate() {
        sum += v;
        while sum >= target {
            result = result.min((right - left + 1) as i32);
            sum -= nums[left];
            left += 1;
        }
    }

    if result == i32::MAX {
        0
    } else {
        result
    }
}
