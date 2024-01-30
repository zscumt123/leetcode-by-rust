fn main() {}

struct Solution;
impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        fn calc(mut num: i32) -> i32 {
            let mut res = 0;
            while num > 0 {
                res += (num % 2);
                num >>= 1;
            }
            res
        }

        let mut result = 0;
        for (i, v) in nums.into_iter().enumerate() {
            if calc(i as i32) == k {
                result += v;
            }
        }
        result
    }

    pub fn sum_indices_with_k_set_bits2(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().enumerate().fold(0, |num, (i, v)| {
            let current = if i.count_ones() == (k as u32) { *v } else { 0 };
            num + current
        })
    }
}
