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
    let res = solve_fun(tree, 1);
    println!("{:?}", res)
}

fn solve_fun(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut res = 0;
    find_small(root, k, 0, &mut res);
    return res;
}

fn find_small(root: Option<Rc<RefCell<TreeNode>>>, k: i32, cur: i32, res: &mut i32) -> i32 {
    if root.is_none() {
        return cur;
    }

    let root_node = root.unwrap();
    let root_borrow = root_node.borrow();

    let left = find_small(root_borrow.left.clone(), k, cur, res);
    if left >= k {
        return left;
    }
    if left + 1 == k {
        // meaning we are on our desired index
        *res = root_borrow.val;
        return k;
    }

    let right = find_small(root_borrow.right.clone(), k, left + 1, res);
    right
}
