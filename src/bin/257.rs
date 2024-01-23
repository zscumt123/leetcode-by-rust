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
use std::collections::VecDeque;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        match root {
            Some(n) => {
                let node = n.borrow_mut();
                let left = node.left.clone();
                let right = node.right.clone();
                if left.is_none() && right.is_none() {
                    return vec![node.val.to_string()];
                }
                let left_res = Solution::binary_tree_paths(left);
                let right_res = Solution::binary_tree_paths(right);
                left_res
                    .iter()
                    .chain(right_res.iter())
                    .map(|v| format!("{}->{}", node.val, v))
                    .collect::<Vec<_>>()
            }
            None => {
                vec![]
            }
        }
    }
}
