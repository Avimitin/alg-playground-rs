struct MyHashSet {
    store: Vec<bool>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            store: vec![false; 1_000_000],
        }
    }

    fn add(&mut self, key: i32) {
        if let Some(val) = self.store.get_mut(key as usize) {
            *val = true
        }
    }

    fn remove(&mut self, key: i32) {
        if let Some(val) = self.store.get_mut(key as usize) {
            *val = false
        }
    }

    fn contains(&self, key: i32) -> bool {
        if let Some(val) = self.store.get(key as usize) {
            *val
        } else {
            false
        }
    }
}
#[test]
fn test() {}
