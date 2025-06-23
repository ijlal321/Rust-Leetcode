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
        val: 6,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
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
    let p = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let res = solve_fun(tree, p, q);
    println!("{:?}", res)
}

fn solve_fun(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    /* how are we finding. we check if this node is left or right.
        if it is, we find other one is below.
               if we found other below, meaning this cur is our common descendent
               else we return what single we found
        if its not then we check on both left and right.
        both left and right will return an option. if both none, then we have nothing in below.
        but if they return something, it can be 2 things, our p/q or main descendent node.

    */
    if root.is_none() {
        return None;
    }

    let root_node = root.unwrap();
    let p_node = p.unwrap();
    let q_node = q.unwrap();

    if root_node.borrow().val == p_node.borrow().val
        || root_node.borrow().val == q_node.borrow().val
    {
        return Some(root_node);
    }

    let left = solve_fun(
        root_node.borrow().left.clone(),
        Some(p_node.clone()),
        Some(q_node.clone()),
    );
    let right = solve_fun(
        root_node.borrow().right.clone(),
        Some(p_node.clone()),
        Some(q_node.clone()),
    );

    if left.is_some() && right.is_some() {
        return Some(root_node);
    } else if left.is_some() {
        return left;
    } else if right.is_some() {
        return right;
    }else {
        return  None;
    }

}
