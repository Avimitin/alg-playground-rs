#[allow(dead_code)]
struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // handle edge cases
        if head.is_none() {
            return None;
        }

        let mut repeat = head.as_ref().unwrap().val - 1;
        let mut new_list = Some(Box::new(ListNode {
            val: repeat,
            next: head,
        }));

        let mut ptr = &mut new_list.as_mut().unwrap().next;

        loop {
            match ptr {
                // if current pointer is the end, drop the entry and return the list
                None => return new_list.unwrap().next.take(),
                // if current node value is as same as the repeat value, drop the current node
                Some(node) if node.val == repeat => *ptr = node.next.take(),
                // if current node value is not a repeat value, but next value is as same as the
                // current node, update the repeat pattern
                Some(node)
                    if node.next.is_some() && node.val == node.next.as_ref().unwrap().val =>
                {
                    repeat = node.val
                }
                // Else, set the pointer to the next node, update the repeat pattern to current
                // node
                Some(node) => {
                    ptr = &mut node.next;
                    if let Some(n) = ptr {
                        repeat = n.val - 1;
                    }
                }
            };
        }
    }
}

#[test]
fn test() {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 5, next: None })),
                        })),
                    })),
                })),
            })),
        })),
    }));

    let result = Solution::delete_duplicates(head);
    dbg!(result);
}
