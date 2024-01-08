fn main() {}
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
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
use std::borrow::Borrow;
use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut node = root;

        while node.is_some() || !stack.is_empty() {
            while let Some(n) = node {
                node = n.borrow_mut().left.clone();
                stack.push(n)
            }

            if let Some(first) = stack.pop() {
                ans.push(first.borrow_mut().val);
                node = first.borrow_mut().right.clone();
            }
        }

        ans
    }
}
