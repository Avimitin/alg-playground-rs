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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut vec = Vec::new();
        Self::traverse(root.clone(), &mut vec);
        if vec.is_empty() {
            return;
        }
        vec.sort_unstable();

        Self::updtree(root, &vec);
    }

    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, s: &mut Vec<i32>) -> Option<()> {
        let node = root?;
        Self::traverse(node.borrow().left.clone(), s);
        s.push(node.borrow().val);
        Self::traverse(node.borrow().right.clone(), s);

        Some(())
    }

    fn updtree(root: &mut Option<Rc<RefCell<TreeNode>>>, v: &[i32]) {
        let mut i = 0;
        Self::fixtree(root, v, &mut i);
    }

    fn fixtree(root: &mut Option<Rc<RefCell<TreeNode>>>, v: &[i32], i: &mut usize) {
        if root.is_none() {
            return;
        }
        let node = root.as_mut().unwrap();
        Self::fixtree(&mut node.borrow_mut().left.clone(), v, i);
        node.borrow_mut().val = v[*i];
        *i += 1;
        Self::fixtree(&mut node.borrow_mut().right.clone(), v, i);
    }
}

#[test]
fn test() {}
