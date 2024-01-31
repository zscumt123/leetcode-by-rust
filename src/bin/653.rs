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
use std::collections::HashSet;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut cache = HashSet::new();
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cache: &mut HashSet<i32>, k: i32) -> bool {
            if let Some(n) = root {
                let node = n.borrow();
                if cache.contains(&node.val) {
                    return true;
                } else {
                    cache.insert(k - node.val);
                    return dfs(node.left.clone(), cache, k) || dfs(node.right.clone(), cache, k);
                }
            }
            return false;
        }
        dfs(root, &mut cache, k)
    }
}
