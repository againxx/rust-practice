use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert(1, "a");
    assert_eq!(map.get(&1).unwrap(), &"a");
}
