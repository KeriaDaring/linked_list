
mod first;

pub use first::List;

pub fn test() {
    let mut list = List::new();
    list.push(1);
}

fn main() {
    test();
    println!("hello");
}

