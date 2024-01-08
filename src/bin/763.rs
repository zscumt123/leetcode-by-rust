fn main() {
    Solution::partition_labels("ababcbacadefegdehijhklij".to_string());
}

/*


给你一个字符串 s 。我们要把这个字符串划分为尽可能多的片段，同一字母最多出现在一个片段中。

注意，划分结果需要满足：将所有划分结果按顺序连接，得到的字符串仍然是 s 。

返回一个表示每个字符串片段的长度的列表。



示例 1：
输入：s = "ababcbacadefegdehijhklij"
输出：[9,7,8]
解释：
划分结果为 "ababcbaca"、"defegde"、"hijhklij" 。
每个字母最多出现在一个片段中。
像 "ababcbacadefegde", "hijhklij" 这样的划分是错误的，因为划分的片段数较少。
示例 2：

输入：s = "eccbbbbdec"
输出：[10]


*/

use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut cache = HashMap::new();
        for (i, char) in s.chars().enumerate() {
            cache.insert(char, i);
        }
        let mut pos = 0;
        let mut prev = 0;
        for (index, char) in s.chars().enumerate() {
            if let Some(cur) = cache.get(&char) {
                pos = pos.max(*cur);

                if index == pos {
                    res.push((pos - prev + 1) as i32);
                    prev = index + 1;
                }
            }
        }
        res
    }
}
