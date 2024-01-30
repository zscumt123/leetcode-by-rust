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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        fn dfs(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (left, right) {
                (Some(left), Some(right)) => {
                    let (lb, rb) = (left.borrow(), right.borrow());
                    return lb.val == rb.val
                        && dfs(lb.left.clone(), rb.right.clone())
                        && dfs(lb.right.clone(), rb.left.clone());
                }
                (None, None) => true,
                _ => false,
            }
        }
        dfs(
            root.clone().unwrap().borrow().left.clone(),
            root.clone().unwrap().borrow().right.clone(),
        )
    }

    pub fn is_symmetric2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(n) = root {
            let mut queue = VecDeque::from(vec![
                n.borrow_mut().left.take(),
                n.borrow_mut().right.take(),
            ]);
            while let (Some(l), Some(r)) = (queue.pop_front(), queue.pop_front()) {
                match (l, r) {
                    (Some(a), Some(b)) => {
                        let l_node = a.borrow();
                        let r_node = b.borrow();

                        if l_node.val != r_node.val {
                            return false;
                        }
                        queue.extend(vec![
                            l_node.left.clone(),
                            r_node.right.clone(),
                            l_node.right.clone(),
                            r_node.left.clone(),
                        ]);
                    }
                    (None, None) => {}
                    _ => return false,
                }
            }
        }

        return true;
    }
}
