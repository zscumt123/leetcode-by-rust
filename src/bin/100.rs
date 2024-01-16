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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let pb = p.borrow();
                let qb = q.borrow();
                pb.val == qb.val
                    && Self::is_same_tree(pb.left.clone(), qb.left.clone())
                    && Self::is_same_tree(pb.right.clone(), qb.right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}
