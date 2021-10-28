#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut curr_node = &head;
        while let Some(node) = curr_node.as_ref() {
            stack.push(node.val);
            curr_node = &node.next;
        }
        let mut result = Vec::new();
        while !stack.is_empty() {
            result.push(stack.pop().unwrap());
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
