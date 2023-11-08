use std::collections::BTreeMap;
use linked_list::test;

fn main() {
    let mut tree = BTreeMap::new();
    tree.insert("xiaoming", 1);
    tree.insert("32", 1);
    tree.insert("42", 1);
    tree.insert("34", 1);

    for i in tree {
        println!("{:?}", i);
    }

    test();
}
