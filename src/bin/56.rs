fn main() {
    let res = Solution::merge(vec![vec![1, 4], vec![4, 5]]);

    println!("res:{:?}", res)
}

pub struct Solution;
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return vec![];
        }
        let mut intervals = intervals;
        intervals.sort();

        let (mut start, mut end) = (intervals[0][0], intervals[0][1]);
        let mut result = vec![];

        intervals.iter().skip(1).for_each(|x| {
            if end < x[0] {
                result.push(vec![start, end]);
                start = x[0]
            }
            end = end.max(x[1]);
        });
        result.push(vec![start, end]);
        result
    }
}
