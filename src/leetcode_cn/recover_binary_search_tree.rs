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

#[derive(Default)]
struct Runtime {
    prev: Option<Rc<RefCell<TreeNode>>>,
    // swap below
    first: Option<Rc<RefCell<TreeNode>>>,
    second: Option<Rc<RefCell<TreeNode>>>,
}

impl Solution {
    #[allow(dead_code)]
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut rt = Runtime::default();
        Self::traverse(root.clone(), &mut rt);

        let first = rt.first.unwrap();
        let second = rt.second.unwrap();

        let tmp = first.borrow().val;
        first.borrow_mut().val = second.borrow().val;
        second.borrow_mut().val = tmp;
    }

    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, rt: &mut Runtime) -> Option<()> {
        let node = root?;
        Self::traverse(node.borrow().left.clone(), rt);
        let val = node.borrow().val;
        if let Some(prev_node) = &rt.prev {
            if val < prev_node.borrow().val {
                if rt.first.is_none() {
                    rt.first = Some(prev_node.clone());
                }

                rt.second = Some(node.clone());
            }
        }

        rt.prev = Some(node.clone());
        Self::traverse(node.borrow().right.clone(), rt);

        Some(())
    }
}

#[test]
fn test() {}
