use crate::map::Map;

pub struct LogMap<A, B> {
    pub kv: Vec<(A, B)>,
}

impl<A, B> LogMap<A, B> {
    pub fn contains_key(&self, key: A) -> bool
    where
        A: PartialEq,
    {
        self.kv.iter().any(|(key_chance, _)| key_chance == &key)
    }
}

impl<A, B> Map<A, B> for LogMap<A, B> {
    fn new() -> Self {
        Self { kv: vec![] }
    }

    fn add(&mut self, key: A, value: B) {
        self.kv.append(&mut vec![(key, value)]);
    }

    fn get(&self, key: A) -> Option<B>
    where
        A: PartialEq + Clone,
        B: Copy,
    {
        for (k, v) in &self.kv {
            if *k == key {
                return Some(*v);
            }
        }
        None
    }

    fn remove(&mut self, key: A) -> Result<(), Box<dyn std::error::Error>>
    where
        A: PartialEq,
    {
        let mut index = -1;
        for (i, (self_key, _)) in self.kv.iter().enumerate() {
            if self_key == &key {
                index = i as isize;
                break;
            }
        }

        if index == -1 {
            Err("Key doesnt not exist".into())
        } else {
            self.remove_index(index as usize);
            Ok(())
        }
    }

    fn remove_index(&mut self, index: usize) {
        self.kv.remove(index);
    }
}
