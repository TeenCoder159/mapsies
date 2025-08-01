use std::error::Error;

pub trait Map<A, B> {
    fn new() -> Self;
    fn add(&mut self, key: A, value: B)
    where
        A: PartialEq,
        B: Copy;
    fn get(&self, key: A) -> Option<B>
    where
        A: PartialEq + Clone,
        B: Copy;
    fn remove(&mut self, key: A) -> Result<(), Box<dyn Error>>
    where
        A: PartialEq;
    fn remove_index(&mut self, index: usize);
}
