#[derive(Default)]
struct CQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl CQueue {
    fn new() -> Self {
        Default::default()
    }

    fn append_tail(&mut self, value: i32) {
        self.stack1.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        match self.stack2.pop() {
            Some(value) => value,
            None => {
                while let Some(old_value) = self.stack1.pop() {
                    self.stack2.push(old_value);
                }
                self.stack2.pop().unwrap_or(-1)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
