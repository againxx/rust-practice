use std::pin::Pin;
use std::ptr::NonNull;
use std::marker::PhantomPinned;

struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);
        let slice = NonNull::from(&boxed.data);
        unsafe {
            let mut_ref: Pin<&mut Self> = boxed.as_mut();
            mut_ref.get_unchecked_mut().slice = slice;
        }
        boxed
    }
}

fn main() {
    let unmoved = Unmovable::new("hello".to_string());
    let mut still_unmoved = unmoved;
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));

    let mut new_unmoved = Unmovable::new("world".to_string());
    std::mem::swap(&mut *new_unmoved, &mut *still_unmoved);
}
