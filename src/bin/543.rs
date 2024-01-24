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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, num: &mut i32) -> i32 {
            match root {
                Some(n) => {
                    let node = n.borrow();
                    let left_res = dfs(node.left.clone(), num);
                    let right_res = dfs(node.right.clone(), num);
                    *num = (*num).max(left_res + right_res);
                    left_res.max(right_res) + 1
                }
                None => 0,
            }
        }
        dfs(root, &mut res);
        res
    }
}
