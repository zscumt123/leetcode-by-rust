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

use std::collections::BTreeMap;
impl ListNode {
    fn append(self, val: i32) -> Self {
        ListNode {
            next: Some(Box::new(ListNode::new(val))),
            ..self
        }
    }
}
struct Solution;
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut chain = head;
        let mut cache = BTreeMap::new();
        while let Some(t) = chain.take() {
            let val = cache.entry(t.val).or_insert(0);
            *val += 1;
            chain = t.next;
        }
        let mut res = vec![];

        for (k, v) in &cache {
            if *v == 1 {
                res.push(*k);
            }
        }
        res.reverse();
        let mut prev = None;
        for i in res {
            let mut node = ListNode::new(i);
            node.next = prev;
            prev = Some(Box::new(node));
        }
        prev
    }

    pub fn delete_duplicates2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut hair = Some(Box::new(ListNode::new(-101)));
        let mut pointer = hair.as_mut().unwrap();
        let mut prev = -102;
        while let Some(mut node) = head {
            //node是当前节点
            //head是下一个
            head = node.next.take();

            if node.val == prev || (head.is_some() && head.as_mut().unwrap().val == node.val) {
                prev = node.val;
            } else {
                prev = node.val;
                pointer.next = Some(node);
                pointer = pointer.next.as_mut().unwrap();
            }
        }

        hair.as_mut().unwrap().next.take()
    }
}
