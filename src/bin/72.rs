fn main() {
    let res = Solution::min_distance("horse".to_string(), "ros".to_string());
    println!("{}", res)
}
struct Solution;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let len1 = word1.len();
        let len2 = word2.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len2 + 1]; len1 + 1];

        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        for i in 0..=len1 {
            dp[i][0] = i as i32;
        }
        for j in 0..=len2 {
            dp[0][j] = j as i32;
        }
        println!("{:?}", dp);

        for i in 1..=len1 {
            for j in 1..=len2 {
                dp[i][j] = if w1[i - 1] == w2[j - 1] {
                    dp[i - 1][j - 1]
                } else {
                    dp[i][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j - 1]) + 1
                };
            }
        }
        dp[len1][len2]
    }
}
