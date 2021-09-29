#[derive(PartialEq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None
        }
    }
}

fn solution(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1_node = l1;
    let mut l2_node = l2;
    let mut result = Some(Box::new(ListNode::new(0)));
    let mut result_node = result.as_mut();
    let mut carry = 0;
    while l1_node.is_some() || l2_node.is_some() || carry != 0 {
        let mut sum = carry;
        if let Some(node) = l1_node {
            sum += node.val;
            l1_node = node.next;
        }
        if let Some(node) = l2_node {
            sum += node.val;
            l2_node = node.next
        }
        carry = sum / 10;
        if let Some(node) = result_node {
            node.next = Some(Box::new(ListNode::new(sum % 10)));
            result_node = node.next.as_mut();
        }
    }
    result.unwrap().next
}

fn main() {
    let l1 = Box::new(ListNode{
        val: 2, next: Some(Box::new(
            ListNode{val: 1, next: Some(
                Box::new(ListNode{val: 4, next: None}))}))});
    let l2 = Box::new(ListNode{
        val: 2, next: Some(Box::new(
            ListNode{val: 1, next: Some(Box::new(
                ListNode{val: 4, next: None}))}))});
    println!("{:?}", solution(Some(l1), Some(l2)));
}
