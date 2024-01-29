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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Solution::dfs(&root, &sub_root)
    }

    pub fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (Some(r), Some(_)) => {
                Solution::isSame(root.clone(), sub_root.clone())
                    || Solution::dfs(&r.borrow().left, &sub_root)
                    || Solution::dfs(&r.borrow().right, &sub_root)
            }
            (None, None) => true,
            _ => false,
        }
    }

    fn isSame(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (t1, t2) {
            (Some(n1), Some(n2)) => {
                n1.borrow().val == n2.borrow().val
                    && Solution::isSame(n1.borrow().left.clone(), n2.borrow().left.clone())
                    && Solution::isSame(n1.borrow().right.clone(), n2.borrow().right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}
