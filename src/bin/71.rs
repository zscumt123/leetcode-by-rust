fn main() {}

use std::collections::VecDeque;
struct Solution;
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        path.split('/').into_iter().for_each(|v| {
            if v == ".." {
                stack.pop();
            } else if v != "" && v != "." {
                stack.push(v);
            }
        });

        format!("/{}", stack.join("/"))
    }
}
