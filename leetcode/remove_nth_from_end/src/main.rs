#[derive(PartialEq, Eq, Clone, Debug)]
pub struct  ListNode {
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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        {
            let mut first_node = &head;
            while let Some(curr_node) = first_node.as_ref() {
                first_node = &curr_node.next;
                len += 1;
            }
        }
        if len == n {
            return head?.next
        }

        let mut second_node = &mut head;
        for _ in 0..(len - n - 1) {
            second_node = &mut second_node.as_mut()?.next;
        }
        second_node.as_mut()?.next = second_node.as_mut()?.next.as_mut()?.next.take();
        head
    }
}

fn main() {
    println!("Hello, world!");
}
