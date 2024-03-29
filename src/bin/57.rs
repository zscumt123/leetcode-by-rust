fn main() {}

/**
 *
 * 给你一个 无重叠的 ，按照区间起始端点排序的区间列表。

在列表中插入一个新的区间，你需要确保列表中的区间仍然有序且不重叠（如果有必要的话，可以合并区间）。



示例 1：

输入：intervals = [[1,3],[6,9]], newInterval = [2,5]
输出：[[1,5],[6,9]]
示例 2：

输入：intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
输出：[[1,2],[3,10],[12,16]]
解释：这是因为新的区间 [4,8] 与 [3,5],[6,7],[8,10] 重叠。
示例 3：

输入：intervals = [], newInterval = [5,7]
输出：[[5,7]]
示例 4：

输入：intervals = [[1,5]], newInterval = [2,3]
输出：[[1,5]]
示例 5：

输入：intervals = [[1,5]], newInterval = [2,7]
输出：[[1,7]]

 *
 *
*/
struct Solution;
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut intervals = intervals;
        intervals.push(new_interval);
        intervals.sort();

        let (mut start, mut end) = (intervals[0][0], intervals[0][1]);
        //intervals = [[1,3],[6,9]], newInterval = [2,5]
        intervals.iter().skip(1).for_each(|x| {
            if end < x[0] {
                res.push(vec![start, end]);
                start = x[0]
            }
            end = end.max(x[1])
        });
        res.push(vec![start, end]);

        res
    }
}
