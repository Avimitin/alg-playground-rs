// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ptr = root.clone();
        while ptr.is_some() {
            match ptr {
                Some(node) if node.borrow().val == val => return Some(node.clone()),
                Some(node) if node.borrow().val < val => ptr = node.borrow().right.clone(),
                Some(node) if node.borrow().val > val => ptr = node.borrow().left.clone(),
                Some(_) => return None, // leetcode should guarantee this will never happen
                None => break,
            }
        }

        None
    }
}
#[test]
fn test() {}
