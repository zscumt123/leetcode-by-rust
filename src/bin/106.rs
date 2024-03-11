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
use std::collections::HashMap;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cache = HashMap::new();
        for (i, v) in inorder.iter().enumerate() {
            cache.insert(*v, i);
        }
        fn dfs(
            in_order: &Vec<i32>,
            post_order: &Vec<i32>,
            in_left: usize,
            in_right: usize,
            post_left: usize,
            post_right: usize,
            cache: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if in_right < in_left || post_right < post_left {
                return None;
            }
            let root = post_order[post_right];

            let root_in_order = cache.get(&root).unwrap();
            let diff = *root_in_order - 1 - in_left;

            Some(Rc::new(RefCell::new(TreeNode {
                val: root,
                left: dfs(
                    in_order,
                    post_order,
                    in_left,
                    root_in_order - 1,
                    post_left,
                    post_left + diff,
                    cache,
                ),
                right: dfs(
                    in_order,
                    post_order,
                    root_in_order + 1,
                    in_right,
                    post_left + diff + 1,
                    post_right - 1,
                    cache,
                ),
            })))
        }
        dfs(
            &inorder,
            &postorder,
            0,
            inorder.len() - 1,
            0,
            postorder.len() - 1,
            &cache,
        )
    }
}
