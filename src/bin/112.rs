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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(n) => {
                let node = n.borrow_mut();
                let extra = target_sum - node.val;
                let left = node.left.clone();
                let right = node.right.clone();
                if extra == 0 && left.is_none() && right.is_none() {
                    return true;
                }

                Solution::has_path_sum(left, extra) || Solution::has_path_sum(right, extra)
            }
            None => false,
        }
    }
}
