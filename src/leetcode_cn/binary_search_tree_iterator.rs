use core::cell::RefCell;
use std::rc::Rc;

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

struct BSTIterator {
    st: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut bstit = Self { st: Vec::new() };
        bstit.traverse(root);
        bstit
    }

    fn traverse(&mut self, mut root: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(node) = root {
            self.st.push(node.clone());
            root = node.borrow().left.clone();
        }
    }

    fn next(&mut self) -> i32 {
        let node = self.st.pop().unwrap();
        let x = node.borrow().val;
        self.traverse(node.borrow().right.clone());
        x
    }

    fn has_next(&mut self) -> bool {
        !self.st.is_empty()
    }
}
