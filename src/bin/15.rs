use std::cmp::Ordering;

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let t = Solution::three_sum(nums);
    println!("{:?}", t);
}

struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = vec![];
        let len = nums.len();
        if len < 3 {
            return result;
        }

        for i in 0..len - 2 {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut j, mut k) = (i + 1, len - 1);

            while j < k {
                match (nums[i] + nums[j] + nums[k]).cmp(&0) {
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[j], nums[k]]);
                        while j < k && nums[j] == nums[j + 1] {
                            j += 1;
                        }
                        while j < k && nums[k] == nums[k - 1] {
                            k -= 1;
                        }
                        j += 1;
                        k -= 1;
                    }
                    Ordering::Greater => {
                        k -= 1;
                    }
                    Ordering::Less => {
                        j += 1;
                    }
                }
            }
        }

        result
    }
}
