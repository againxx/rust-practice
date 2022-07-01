#[derive(Debug)]
enum List {
    Cons(RefCell<i32>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let a = Rc::new(Cons(RefCell::new(5), Rc::new(Nil)));
    let b = Cons(RefCell::new(6), Rc::clone(&a));
    let c = Cons(RefCell::new(10), Rc::clone(&a));

    if let Cons(value, _) = &*a {
        *value.borrow_mut() = -1;
    }

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
