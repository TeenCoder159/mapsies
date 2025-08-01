use std::error::Error;

#[derive(Debug)]
/// Linear map:
pub struct LinearMap<T, S> {
    pub key: Vec<T>,
    pub value: Vec<S>,
}

impl<T, S> LinearMap<T, S> {
    pub fn new() -> Self {
        Self {
            key: vec![],
            value: vec![],
        }
    }

    pub fn add(&mut self, key: T, val: S) -> Result<(), Box<dyn std::error::Error>>
    where
        T: PartialEq,
    {
        if self.key.contains(&key) {
            return Err("Key already exists".into());
        }

        self.key.push(key);
        self.value.push(val);

        Ok(())
    }

    pub fn get(&self, key: &T) -> Option<&S>
    where
        T: PartialEq,
    {
        let mut i = 0;
        let mut found = false;

        for val in &self.key {
            if val == key {
                found = true;
                break;
            }
            i += 1;
        }

        if found {
            Some(self.value.get(i).unwrap())
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: T) -> Result<(), Box<dyn Error>>
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

    pub fn remove_index(&mut self, index: usize) {
        self.key.remove(index);
        self.value.remove(index);
    }
}
