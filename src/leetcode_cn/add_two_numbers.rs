#[allow(dead_code)]
struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut left: i32 = 0;

        if l1.is_none() && l2.is_none() {
            return None;
        }

        let mut head = Some(Box::new(ListNode::new(-1)));
        let mut ptr1 = &l1;
        let mut ptr2 = &l2;
        let mut ans = &mut head.as_mut().unwrap().next;

        loop {
            match ptr1 {
                None => break,
                Some(node) => {
                    let val;
                    let sum;
                    // if two node exist
                    if let Some(node2) = ptr2 {
                        sum = node.val + node2.val + left;
                        ptr2 = &node2.next; // update the node2
                    } else {
                        // if the l2 has no node
                        sum = node.val + left;
                    }

                    if sum > 9 {
                        left = sum / 10;
                        val = sum - 10;
                    } else {
                        left = 0;
                        val = sum;
                    }

                    ptr1 = &node.next;
                    *ans = Some(Box::new(ListNode::new(val)));
                    ans = &mut ans.as_mut().unwrap().next;
                }
            }
        } // the list 1 is consumed

        loop {
            match ptr2 {
                None => break,
                Some(node) => {
                    let val;
                    let sum = node.val + left;
                    if sum > 9 {
                        left = sum / 10;
                        val = sum - 10;
                    } else {
                        left = 0;
                        val = sum;
                    }
                    ptr2 = &node.next;
                    *ans = Some(Box::new(ListNode::new(val)));
                    ans = &mut ans.as_mut().unwrap().next;
                }
            }
        }

        if left > 0 {
            *ans = Some(Box::new(ListNode::new(left)));
        }

        head.unwrap().next
    }
}

#[test]
fn test() {}
