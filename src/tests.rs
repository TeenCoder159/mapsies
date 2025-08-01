#[cfg(test)]
mod tests {
    use crate::map::Map;

    #[test]
    fn linmap_test() {
        use crate::linmap::*;

        let mut lm: LinearMap<i32, &str> = LinearMap::new();

        lm.add(0, "something");

        assert_eq!(lm.get(0), Some("something"));
        assert_eq!(lm.get(2), None);
    }

    #[test]
    fn logmap_test() {
        use crate::logmap::LogMap;

        let mut lm: LogMap<i32, &str> = LogMap::new();

        lm.add(0, "something");

        assert_eq!(lm.get(0), Some("something"));
        assert_eq!(lm.get(2), None);
    }
}
