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
//1 2 3 4 5
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut l: *const _ = &head;
        let mut d = head.as_mut().unwrap();
        let mut i = 0;
        while let Some(sl) = unsafe { &*l } {
            l = &sl.next;
            if i > n {
                d = d.next.as_mut().unwrap()
            }
            i += 1;
        }
        if i == n {
            return head.unwrap().next;
        }
        d.next = d.next.take().unwrap().next;
        head
    }
}
