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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(n) => {
                let node = n.borrow();
                let left = node.left.clone();
                let right = node.right.clone();
                let mut sum = 0;
                if let Some(l_node) = &left {
                    let next_node = l_node.borrow();
                    if let (None, None) = (next_node.left.clone(), next_node.right.clone()) {
                        sum += next_node.val
                    }
                }
                return sum
                    + Solution::sum_of_left_leaves(left)
                    + Solution::sum_of_left_leaves(right);
            }
            None => 0,
        }
    }
}
