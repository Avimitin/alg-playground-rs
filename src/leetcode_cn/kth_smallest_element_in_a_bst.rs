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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut store = Vec::new();
        Self::dfs(root, &mut store);

        store[(k - 1) as usize]
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, s: &mut Vec<i32>) -> Option<()> {
        let node = root?;
        Self::dfs(node.borrow().left.clone(), s);
        s.push(node.borrow().val);
        Self::dfs(node.borrow().right.clone(), s);
        Some(())
    }
}

#[test]
fn test() {}
