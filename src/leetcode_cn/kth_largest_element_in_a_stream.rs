#[allow(dead_code)]
struct KthLargest {
    heap: Vec<i32>,
    cap: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    #[allow(dead_code)]
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        if nums.len() <= k as usize {
            return Self {
                heap: {
                    nums.sort_unstable();
                    nums
                },
                cap: k as usize,
            };
        }

        // a array with k (-10^4 - 1) element
        let mut kth = Self {
            heap: vec![-(10_i32.pow(4)) - 1],
            cap: k as usize,
        };

        for n in nums {
            kth.push(n);
        }

        kth
    }

    #[allow(dead_code)]
    fn add(&mut self, val: i32) -> i32 {
        self.push(val);
        self.heap[0]
    }

    #[allow(dead_code)]
    fn push(&mut self, val: i32) {
        if self.heap.is_empty() || self.heap.len() < self.cap {
            let res = self.heap.binary_search(&val);
            match res {
                Ok(i) => self.heap.insert(i, val),
                Err(i) => self.heap.insert(i, val),
            }
        } else if val > self.heap[self.heap.len() - 1] {
            self.heap.remove(0);
            self.heap.push(val);
        } else {
            let res = self.heap.binary_search(&val);
            match res {
                Ok(i) => {
                    self.heap.remove(0);
                    self.heap.insert(i, val);
                }
                Err(i) => {
                    if i == 0 && self.heap[i] < val {
                        self.heap.remove(0);
                        self.heap.insert(0, val);
                    } else if i != 0 {
                        self.heap.remove(0);
                        self.heap.insert(i - 1, val);
                    }
                }
            }
        }
    }
}

///
///Your KthLargest object will be instantiated and called as such:
///let obj = KthLargest::new(k, nums);
///let ret_1: i32 = obj.add(val);
///
#[test]
fn test() {
    let mut k = KthLargest::new(3, vec![5, -1]);
    assert_eq!(k.add(2), -1);
    assert_eq!(k.add(1), 1);
    assert_eq!(k.add(-1), 1);
    assert_eq!(k.add(3), 2);
    assert_eq!(k.add(4), 3);
}
