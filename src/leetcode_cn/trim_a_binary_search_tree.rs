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
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root?;
        if node.borrow().val < low {
            return Self::trim_bst(node.borrow().right.clone(), low, high);
        } else if node.borrow().val > high {
            return Self::trim_bst(node.borrow().left.clone(), low, high);
        }

        let nnode = TreeNode {
            val: node.borrow().val,
            left: Solution::trim_bst(node.borrow().left.clone(), low, high),
            right: Solution::trim_bst(node.borrow().right.clone(), low, high),
        };

        Some(Rc::new(RefCell::new(nnode)))
    }
}

#[test]
fn test() {}
