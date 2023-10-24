fn main() {}

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
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut chain = Some(Box::new(ListNode::new(-1)));
        let mut first = l1.as_ref();
        let mut second = l2.as_ref();

        let mut head = chain.as_mut();
        let mut carry = 0;
        while first.is_some() || second.is_some() {
            let mut sum = carry;
            if let Some(v1) = first {
                sum += v1.val;
                first = v1.next.as_ref();
            }
            if let Some(v2) = second {
                sum += v2.val;
                second = v2.next.as_ref();
            }
            carry = sum / 10;
            head.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            head = head.unwrap().next.as_mut();
        }

        if carry > 0 {
            head.unwrap().next = Some(Box::new(ListNode::new(carry)));
        }

        chain.unwrap().next
    }
}
