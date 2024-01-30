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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn get_height(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(n) => {
                    let inner = n.borrow_mut();
                    get_height(inner.left.clone()).max(get_height(inner.right.clone())) + 1
                }
                None => 0,
            }
        }

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match root {
                Some(n) => {
                    let node = n.borrow_mut();
                    let height =
                        (get_height(node.left.clone()) - get_height(node.right.clone())).abs();
                    dfs(node.left.clone()) && dfs(node.right.clone()) && height <= 1
                }
                None => return true,
            }
        }
        dfs(root)
    }

    pub fn is_balanced2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn get_height(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(n) => {
                    let inner = n.borrow_mut();
                    let l_h = get_height(inner.left.clone());
                    let r_h = get_height(inner.right.clone());
                    if l_h == -1 || r_h == -1 || (l_h - r_h).abs() > 1 {
                        -1
                    } else {
                        l_h.max(r_h) + 1
                    }
                }
                None => 0,
            }
        }
        get_height(root) >= 0
    }
}
