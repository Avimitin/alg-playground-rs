struct MyHashSet {
    store: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self { store: Vec::new() }
    }

    fn add(&mut self, key: i32) {
        if let Err(i) = self.store.binary_search(&key) {
            self.store.insert(i, key)
        }
    }

    fn remove(&mut self, key: i32) {
        if let Ok(i) = self.store.binary_search(&key) {
            self.store.remove(i);
        }
    }

    fn contains(&self, key: i32) -> bool {
        self.store.binary_search(&key).is_ok()
    }
}
#[test]
fn test() {}
