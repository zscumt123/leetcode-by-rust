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
use std::collections::VecDeque;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                Some(n) => {
                    let node = n.borrow_mut();
                    let l_depth = dfs(node.left.clone());
                    let r_depth = dfs(node.right.clone());
                    match (l_depth, r_depth) {
                        (0, 0) => 1,
                        (0, _) => r_depth + 1,
                        (_, 0) => l_depth + 1,
                        _ => l_depth.min(r_depth) + 1,
                    }
                }
                None => 0,
            }
        }

        dfs(root)
    }

    pub fn min_depth2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut queue = VecDeque::new();
        queue.push_back((root, 1));
        while let Some((n, depth)) = queue.pop_front() {
            let node = n.unwrap().clone();
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            if left.is_none() && right.is_none() {
                return depth;
            }
            if left.is_some() {
                queue.push_back((left, depth + 1))
            }
            if right.is_some() {
                queue.push_back((right, depth + 1))
            }
        }
        return 0;
    }
}
