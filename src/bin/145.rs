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
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/**
 *    1
 * 2    3
 *4 5  6 7
 *
 *
 * [1,2,4,5,3,6,7]
 * [4,5,2, 6,7,3,1]
 *
 *
 *
*/

struct Solution;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![root];

        while let Some(t) = stack.pop() {
            if let Some(node) = t {
                result.push(node.borrow().val);
                stack.push(node.borrow().left.clone());
                stack.push(node.borrow().right.clone());
            }
        }
        result.reverse();
        result
    }
}
