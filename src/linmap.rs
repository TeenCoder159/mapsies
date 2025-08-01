use std::error::Error;

use crate::map::Map;

#[derive(Debug)]
/// Linear map:
pub struct LinearMap<T, S> {
    pub key: Vec<T>,
    pub value: Vec<S>,
}

impl<T, S> Map<T, S> for LinearMap<T, S> {
    fn new() -> Self {
        Self {
            key: vec![],
            value: vec![],
        }
    }

    fn add(&mut self, key: T, val: S)
    where
        T: PartialEq,
        S: Copy,
    {
        if self.key.contains(&key) {
            return;
        }

        self.key.push(key);
        self.value.push(val);
    }

    fn get(&self, key: T) -> Option<S>
    where
        T: PartialEq,
        S: Copy,
    {
        let mut i = 0;
        let mut found = false;

        for val in &self.key {
            if *val == key {
                found = true;
                break;
            }
            i += 1;
        }

        if found {
            Some(*self.value.get(i).unwrap())
        } else {
            None
        }
    }

    fn remove(&mut self, key: T) -> Result<(), Box<dyn Error>>
    where
        T: PartialEq,
    {
        let mut kv = -1;
        for (i, val) in self.key.iter().enumerate().collect::<Vec<_>>() {
            if val == &key {
                kv = i as isize;
            }
        }
        if kv == -1 {
            Err("Key doesnt not exist".into())
        } else {
            self.remove_index(kv as usize);
            Ok(())
        }
    }

    fn remove_index(&mut self, index: usize) {
        self.key.remove(index);
        self.value.remove(index);
    }
}
