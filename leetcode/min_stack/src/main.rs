#[derive(Default)]
struct MinStack {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        self.stack1.push(x);
        match self.stack2.last() {
            Some(value) => {
                if *value >= x {
                    self.stack2.push(x);
                }
            },
            None => self.stack2.push(x),
        }
    }

    fn pop(&mut self) {
        match self.stack1.pop() {
            Some(value) => {
                if let Some(min_value) = self.stack2.last() {
                    if *min_value == value {
                        self.stack2.pop();
                    }
                }
            },
            None => (),
        }

    }

    fn top(&self) -> i32 {
        *self.stack1.last().unwrap()
    }

    fn min(&self) -> i32 {
        *self.stack2.last().unwrap()
    }
}

fn main() {

}
