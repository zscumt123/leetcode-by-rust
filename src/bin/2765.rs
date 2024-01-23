use core::num;

fn main() {
    let res = Solution::alternating_subarray(vec![2, 3, 4, 3, 4]);
    println!("res:{}", res) //2 3 2 3 5[21,9,5]
}

struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut prev_index = 0;
        let mut result: i32 = -1;
        let len = nums.len();
        let mut flag = 1;
        let mut i = 1;
        while i < len {
            let prev = nums[i - 1];
            let current = nums[i];
            let diff = current - prev;
            if diff != flag {
                if i - prev_index > 1 {
                    result = result.max((i - prev_index) as i32);
                }
                prev_index = if diff == 1 { i - 1 } else { i };
                flag = if diff == 1 { 1 } else { -1 };
            }
            flag = -flag;
            i += 1;
        }
        if i - prev_index > 1 {
            result = result.max((i - prev_index) as i32);
        }
        result
    }
}
