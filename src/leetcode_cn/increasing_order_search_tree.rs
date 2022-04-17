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

impl Solution {
    #[allow(dead_code)]
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v = Vec::new();
        Self::dfs(root, &mut v);

        let mut ptr = None;

        for num in v.iter().rev() {
            let mut tr = TreeNode::new(*num);
            tr.right = ptr.clone();
            ptr = Some(Rc::new(RefCell::new(tr)));
        }

        ptr
    }

    fn dfs(n: Option<Rc<RefCell<TreeNode>>>, s: &mut Vec<i32>) -> Option<()> {
        let node = n?;
        Self::dfs(node.borrow().left.clone(), s);
        s.push(node.borrow().val);
        Self::dfs(node.borrow().right.clone(), s);
        Some(())
    }
}

#[test]
fn test() {}
