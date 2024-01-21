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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(start: i32, end: i32, list: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if start > end {
                return None;
            }
            let mid = (start + end) / 2;
            let node = Rc::new(RefCell::new(TreeNode::new(list[mid as usize])));

            node.borrow_mut().left = dfs(start, mid - 1, list);
            node.borrow_mut().right = dfs(mid + 1, end, list);
            println!("{:?}", node);
            return Some(node);
        }

        return dfs(0, (nums.len() - 1) as i32, &nums);
    }
}
