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

fn main() {
    let tree_1 = Some(Rc::new(RefCell::new(TreeNode {
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

    let tree_2 = Some(Rc::new(RefCell::new(TreeNode {
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
    })));
    let res = solve_fun(tree_1, tree_2);
    println!("{:?}", res)
}

fn solve_fun(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if sub_root.is_none() {
        return true;
    }
    if root.is_none() {
        return false;
    }

    if same_tree(&root, &sub_root) {
        return true;
    }

    let root = root.unwrap();
    let sub_root = sub_root.unwrap();

    return solve_fun(root.borrow_mut().left.take(), Some(Rc::clone(&sub_root)))
        || solve_fun(root.borrow_mut().right.take(), Some(Rc::clone(&sub_root)));
}

fn same_tree(
    root: &Option<Rc<RefCell<TreeNode>>>,
    sub_root: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    
    if root.is_none() && sub_root.is_none() {
        return true;
    }
    if root.is_some()
        && sub_root.is_some()
        && root.as_ref().unwrap().borrow().val == sub_root.as_ref().unwrap().borrow().val
    {
        return same_tree(&root.as_ref().unwrap().borrow().left, &sub_root.as_ref().unwrap().borrow().left)
            && same_tree(&root.as_ref().unwrap().borrow().right, &sub_root.as_ref().unwrap().borrow().right);
    }

    false
}
