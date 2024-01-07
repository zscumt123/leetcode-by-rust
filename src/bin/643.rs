fn main() {
    let rs = find_max_average(vec![0, 4, 0, 3, 2], 1);
    println!("{}", rs)
}

/**
 *
 * 给你一个由 n 个元素组成的整数数组 nums 和一个整数 k 。

请你找出平均数最大且 长度为 k 的连续子数组，并输出该最大平均数。

任何误差小于 10-5 的答案都将被视为正确答案。



示例 1：

输入：nums = [1,12,-5,-6,50,3], k = 4
输出：12.75
解释：最大平均数 (12-5-6+50)/4 = 51/4 = 12.75
示例 2：

输入：nums = [5], k = 1
输出：5.00000
 *
 *
*/

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let (len, k) = (nums.len(), k as usize);
    let mut sum = nums.iter().take(k as usize).sum::<i32>();
    let mut large = sum;
    for i in k..len {
        sum = sum + nums[i] - nums[i - k];
        large = large.max(sum);
    }
    large as f64 / k as f64
}
