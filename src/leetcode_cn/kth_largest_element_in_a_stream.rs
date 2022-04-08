#[allow(dead_code)]
struct KthLargest {
    queue: Vec<i32>,
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
                queue: {
                    nums.sort_unstable();
                    nums
                },
                cap: k as usize,
            };
        }

        // a array with k (-10^4 - 1) element
        let mut kth = vec![-(10_i32.pow(4)) - 1; k as usize];
        for n in nums {
            Self::handle(&mut kth, n, k as usize);
        }

        Self { queue: kth, cap: k as usize }
    }

    #[allow(dead_code)]
    fn add(&mut self, val: i32) -> i32 {
        Self::handle(&mut self.queue, val, self.cap);
        self.queue[0]
    }

    #[allow(dead_code)]
    fn handle(v: &mut Vec<i32>, val: i32, cap: usize) {
        if v.is_empty() || v.len() < cap {
            let res = v.binary_search(&val);
            match res {
                Ok(i) => v.insert(i, val),
                Err(i) => v.insert(i, val),
            }
        } else if val > v[v.len() - 1] {
            v.remove(0);
            v.push(val);
        } else {
            let res = v.binary_search(&val);
            match res {
                Ok(i) => {
                    v.remove(0);
                    v.insert(i, val);
                }
                Err(i) => {
                    if i == 0 && v[i] < val {
                        v.remove(0);
                        v.insert(0, val);
                    } else if i != 0 {
                        v.remove(0);
                        v.insert(i - 1, val);
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
