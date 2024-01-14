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
struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = head.as_mut();
        while let Some(t) = current.take() {
            if let Some(n) = t.next.as_mut() {
                if t.val == n.val {
                    t.next = n.next.take();
                } else {
                    break;
                }
                current = t.next.as_mut();
            }
        }

        head
    }
}
