#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        Self::traverse(root.clone(), &mut sum);
        root
    }

    fn traverse(n: Option<Rc<RefCell<TreeNode>>>, s: &mut i32) -> Option<()> {
        let node = n?;
        Self::traverse(node.borrow().right.clone(), s);
        *s += node.borrow().val;
        node.borrow_mut().val = *s;
        Self::traverse(node.borrow().left.clone(), s);

        Some(())
    }
}

#[test]
fn test() {}
