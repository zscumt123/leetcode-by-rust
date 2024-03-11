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
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut result = 0;
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, result: &mut i32) {
            if let Some(t) = root {
                let node = t.borrow();
                if node.val >= low && node.val <= high {
                    *result += node.val
                }
                dfs(node.left.clone(), low, high, result);
                dfs(node.right.clone(), low, high, result);
            }
        }
        dfs(root, low, high, &mut result);

        result
    }
}
