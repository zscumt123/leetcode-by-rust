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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![root];
        let mut res = vec![];
        while let Some(node) = stack.pop() {
            if let Some(inner) = node {
                res.push(inner.borrow().val);
                stack.extend([inner.borrow().right.clone(), inner.borrow().left.clone()]);
            }
        }
        res
    }
}
