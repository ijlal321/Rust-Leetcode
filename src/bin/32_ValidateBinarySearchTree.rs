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
use core::f64;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));
    let res = solve_fun(tree);
    println!("{:?}", res)
}

fn solve_fun(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    return valid(root, f64::NEG_INFINITY, f64::INFINITY);
}

fn valid(root: Option<Rc<RefCell<TreeNode>>>, left: f64, right: f64) -> bool {
    if root.is_none() {
        return true;
    }

    let node = root.unwrap();
    let node_ref = node.borrow();

    if !(left < (node_ref.val as f64) && (node_ref.val as f64) < right) {
        return false;
    }

    return valid(node_ref.left.clone(), left, node_ref.val as f64)
        && valid(node_ref.right.clone(), node_ref.val as f64, right);

}
