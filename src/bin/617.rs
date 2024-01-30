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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            root1: Option<Rc<RefCell<TreeNode>>>,
            root2: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match (root1, root2) {
                (Some(r1), Some(r2)) => {
                    let (n1, n2) = (r1.borrow(), r2.borrow());
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: n1.val + n2.val,
                        left: dfs(n1.left.clone(), n2.left.clone()),
                        right: dfs(n1.right.clone(), n2.right.clone()),
                    })))
                }
                (None, None) => None,
                (None, s) => s,
                (s, None) => s,
            }
        }

        dfs(root1, root2)
    }
}
