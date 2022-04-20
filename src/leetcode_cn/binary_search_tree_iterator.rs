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
    st: Vec<i32>,
    i: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut bstit = Self {
            st: Vec::new(),
            i: 0,
        };
        bstit.traverse(root);
        bstit
    }

    fn traverse(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> Option<()> {
        let node = root?;
        self.traverse(node.borrow().left.clone());
        self.st.push(node.borrow().val);
        self.traverse(node.borrow().right.clone());
        Some(())
    }

    fn next(&mut self) -> i32 {
        self.i += 1;
        self.st[self.i - 1]
    }

    fn has_next(&mut self) -> bool {
        self.i < self.st.len()
    }
}
