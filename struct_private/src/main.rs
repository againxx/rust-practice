struct Catcher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Catcher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Catcher<T> {
        Catcher {
            calculation,
            value: None,
        }
    }
}

fn main() {
    let mut catcher = Catcher::new(|num| num);
    catcher.value = Some(1);
    println!("{:?}", catcher.value);
}
