struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Solution {
    fn help(mut a: Box<ListNode>, mut b: Box<ListNode>) -> Box<ListNode> {
        let mut res; let mut carry = 0;
        let mut ret = Box::new(ListNode::new(-1));
        let mut tp = ret.as_mut();

        while a.val != -1 || b.val != -1 || carry != 0 {
            res = a.val.max(0) + b.val.max(0) + carry;
            carry = res / 10;
            res %= 10;

            if tp.val == -1 { tp.val = res; }
            else {
                while let Some(ref mut next_node) = tp.next { tp = next_node; }
                tp.next = Some(Box::new(ListNode::new(res)));
            }

            a = a.next.unwrap_or_else(|| Box::new(ListNode::new(-1)));
            b = b.next.unwrap_or_else(|| Box::new(ListNode::new(-1)));
        }

        ret
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Self::help(l1.unwrap(),l2.unwrap()))
    }
}

fn main() {
    assert_eq!(
        Some(Box::new(
            ListNode {
                val: 7,
                next: Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode::new(8))) }))
            }
        )),
        Solution::add_two_numbers(Some(Box::new(
            ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode::new(3))) }))
            }
        )), Some(Box::new(
            ListNode {
                val: 5,
                next: Some(Box::new(ListNode { val: 6, next: Some(Box::new(ListNode::new(4))) }))
            }
        )))
    );
}
