use std::cmp::Ordering;

fn main() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    let res = Solution::three_sum_closest(nums, target);
    println!("{:?}", res)
}
/*
给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。

返回这三个数的和。

假定每组输入只存在恰好一个解。

输入：nums = [-1,2,1,-4], target = 1
输出：2
解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。

*/
struct Solution;
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let (mut result, len) = (i32::MAX, nums.len());
        for i in 0..len - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut j, mut k) = (i + 1, len - 1);

            while j < k {
                let current = nums[i] + nums[j] + nums[k];
                if target == current {
                    return target;
                }
                if (current - target).abs() < (result - target).abs() {
                    result = current
                }

                match current.cmp(&target) {
                    Ordering::Greater => {
                        while j < k && nums[k] == nums[k - 1] {
                            k -= 1;
                        }
                        k -= 1;
                    }
                    Ordering::Less => {
                        while j < k && nums[j] == nums[j + 1] {
                            j += 1;
                        }
                        j += 1;
                    }
                    _ => unreachable!(),
                }
            }
        }
        return result;
    }
}
