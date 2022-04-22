struct MyHashMap {
    store: Vec<[i32; 2]>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        Self {
            store: vec![[0, -1]; 1_000_001],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(val) = self.store.get_mut(key as usize) {
            val[0] = 1;
            val[1] = value;
        }
    }

    fn get(&self, key: i32) -> i32 {
        if let Some(val) = self.store.get(key as usize) {
            if val[0] != 0 {
                return val[1];
            }
        }

        return -1;
    }

    fn remove(&mut self, key: i32) {
        if let Some(val) = self.store.get_mut(key as usize) {
            val[0] = 0;
        }
    }
}

#[test]
fn test() {}
