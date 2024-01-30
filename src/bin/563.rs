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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> i32 {
            match root {
                Some(n) => {
                    let node = n.borrow();
                    let val = node.val;
                    let left = dfs(node.left.clone(), sum);
                    let right = dfs(node.right.clone(), sum);
                    *sum += (left - right).abs();
                    val + right + left
                }
                None => 0,
            }
        }
        let mut result = 0;
        dfs(root, &mut result)
    }
}
