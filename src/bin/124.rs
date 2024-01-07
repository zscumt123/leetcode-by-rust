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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
            if let Some(node) = root {
                let left = 0.max(helper(&node.borrow().left, max_sum));
                let right = 0.max(helper(&node.borrow().right, max_sum));

                *max_sum = (*max_sum).max(node.borrow().val + left + right);

                node.borrow().val + left.max(right)
            } else {
                0
            }
        }
        helper(&root, &mut max_sum);
        max_sum
    }
}
