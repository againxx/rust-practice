use std::sync::Mutex;

fn main() {
    let num = Mutex::new(5);
    {
        let mut m = num.lock().unwrap();
        *m = 6;
    }
}
