#[cfg(test)]
mod tests {
    #[test]
    fn linmap_test() {
        use crate::linmap::*;

        let mut lm: LinearMap<i32, String> = LinearMap::new();

        lm.add(0, "something".to_string())
            .expect("This shouldn't be failing");

        assert_eq!(lm.get(&0), Some(&"something".to_string()));
        assert_eq!(lm.get(&2), None);
    }
}
