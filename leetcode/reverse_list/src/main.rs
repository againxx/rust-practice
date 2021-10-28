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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;
        let mut curr_node = head;
        while let Some(mut tmp) = curr_node {
            curr_node = tmp.next.take();
            tmp.next = new_head;
            new_head = Some(tmp);
        }
        new_head
    }
}

fn main() {
    println!("Hello, world!");
}
