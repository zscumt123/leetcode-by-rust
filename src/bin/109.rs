fn main() {}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
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
use std::i32;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut list = vec![];
        let mut head = head;
        while let Some(mut n) = head {
            list.push(n.val);
            head = n.next.take();
        }

        fn build(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if (nums.len() >= 1) {
                let mid = nums.len() / 2;
                Some(Rc::new(RefCell::new(TreeNode {
                    val: nums[mid],
                    left: build(&nums[0..mid]),
                    right: build(&nums[mid + 1..]),
                })))
            } else {
                None
            }
        }

        build(&list)
    }
}
