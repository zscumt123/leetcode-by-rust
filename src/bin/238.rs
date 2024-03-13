fn main() {
    let arr = vec![1, 2, 3, 4];
    let rs = Solution::product_except_self(arr);
    println!("{:?}", rs);
}
struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        let (mut left, mut right) = (Vec::with_capacity(len), Vec::with_capacity(len));
        let (mut left_sum, mut right_sum) = (1, 1);
        for val in nums.iter() {
            left.push(left_sum);
            left_sum *= val;
        }
        for (i, val) in nums.iter().enumerate().rev() {
            right.push(right_sum);
            right_sum *= *val;
        }

        let mut result: Vec<i32> = Vec::with_capacity(len);
        for i in 0..len {
            result.push(left[i] * right[len - i - 1])
        }
        result
    }
}
