
pub mod first;
pub mod second;
pub mod third;

pub use first::List;

pub fn test() {
    let mut list = List::new();
    list.push(1);
}



