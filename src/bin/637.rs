fn main() {
    let a: f64 = 2147483647f64 + 2147483647f64;
    println!("{}", a / 2 as f64)
}
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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut queue = VecDeque::from(vec![root]);
        let mut result = vec![];
        while !queue.is_empty() {
            let (len, mut sum) = (queue.len(), 0f64);
            for _ in 0..len {
                if let Some(Some(t)) = queue.pop_front() {
                    let node = t.borrow();
                    sum += node.val as f64;
                    if node.left.is_some() {
                        queue.push_back(node.left.clone())
                    }
                    if node.right.is_some() {
                        queue.push_back(node.right.clone())
                    }
                }
            }
            result.push(sum / len as f64);
        }

        result
    }
}
