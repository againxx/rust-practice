#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {val, next: None}
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1_current = l1;
        let mut l2_current = l2;
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut current = result.as_mut();
        let mut carry = 0;

        while l1_current.is_some() || l2_current.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1_current {
                sum += node.val;
                l1_current = node.next;
            }
            if let Some(node) = l2_current {
                sum += node.val;
                l2_current = node.next;
            }

            carry = sum / 10;

            if let Some(node) = current {
                node.next = Some(Box::new(ListNode::new(sum % 10)));
                current = node.next.as_mut();
            }
        }
        result.unwrap().next
    }
}

fn main() {
    let mut l1 = Box::new(ListNode::new(2));
    l1.next = Some(Box::new(ListNode::new(4)));
    if let Some(mut node) = l1.next {
        node.next = Some(Box::new(ListNode::new(3)));
        l1.next = Some(node);
    }

    let mut l2 = Box::new(ListNode::new(5));
    l2.next = Some(Box::new(ListNode::new(6)));
    if let Some(mut node) = l2.next {
        node.next = Some(Box::new(ListNode::new(4)));
        l2.next = Some(node);
    }

    let result_l = Solution::add_two_numbers(Some(l1), Some(l2));
    println!("{:?}", result_l);
}
