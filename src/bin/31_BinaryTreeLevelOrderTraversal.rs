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
    let res = solve_fun(tree);
    println!("{:?}", res)
}

fn solve_fun(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }
    let mut res = vec![];
    let mut queue = std::collections::VecDeque::new();

    queue.push_back(root.unwrap());

    while queue.len() > 0 {
        let mut new_vec: Vec<i32> = vec![];

        for _ in 0..queue.len() {
            let temp_pop = queue.pop_front().unwrap();
            let temp = temp_pop.borrow();

            if temp.left.is_some() {
                queue.push_back(temp.left.clone().unwrap());
            }
            if temp.right.is_some() {
                queue.push_back(temp.right.clone().unwrap());
            }
            new_vec.push(temp.val);
        }
        res.push(new_vec);
    }

    res
}
