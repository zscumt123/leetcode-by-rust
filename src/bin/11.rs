fn main() {
    let h = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let res = Solution::max_area(h);
    println!("res:{}", res);
}

struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while right > left {
            let area = (height[left].min(height[right])) as usize * (right - left);
            result = result.max(area as i32);
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }

        result
    }
}
