fn main() {}
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut prev = i32::MAX;
        let mut result = i32::MAX;
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut i32, result: &mut i32) {
            if let Some(n) = root {
                let node = n.borrow();
                let left = node.left.clone();
                let right = node.right.clone();
                dfs(left, prev, result);
                let diff = (node.val - *prev).abs();
                *result = (*result).min(diff);
                *prev = node.val;
                dfs(right, prev, result)
            }
        }

        dfs(root, &mut prev, &mut result);

        result
    }
}
