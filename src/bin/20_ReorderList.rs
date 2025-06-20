use std::hint::unreachable_unchecked;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    let mut dummy_linked_list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    let _ = solve_fun(&mut dummy_linked_list);
    println!("{:?}", dummy_linked_list);
}

// with making 2 separate lists, then reversing 2nd one
fn solve_fun(head: &mut Option<Box<ListNode>>) {
    // count length of list
    let mut count = 0;
    let mut list = &*head; // creating an immu ref, allowed bcz we are not using mut one in this scope

    while list.is_some(){
      list = & list.as_ref().unwrap().next;
      count += 1;
    };


    // Reach the middle of the list in order to split in to two lists
    let mut half = head.as_mut();

    for _ in 0..(count / 2){
      half = half.unwrap().next.as_mut();
    };

    let half = half.unwrap().next.take();
    let mut prev = None;

    // merge the two lists, tail points to the node in the first list
    // that has to be disconnected in order to put a node from the reversed
    // list in its place. Then it's reattached as "next" of the new node
    let mut tail = match head.as_mut() {
        // SAFETY: we know there is node at HEAD
        None => unsafe { unreachable_unchecked() },
        Some(node) => &mut node.next,
    };

    while tail.is_some() && prev.is_some() {
        // SAFETY: We know there is a prev because we already checked it
        let mut rev = prev.take().unwrap();
        prev = rev.next.take();

        rev.next = tail.take();
        *tail = Some(rev);
        tail = &mut tail.as_mut().unwrap().next;
        if let Some(node) = tail {
            tail = &mut node.next;
        }
    }
}

// with making 2 separate lists, then reversing 2nd one
fn Master_solve_fun(head: &mut Option<Box<ListNode>>) {
    // count length of list
    let mut count = 0;
    let mut list = &*head;

    while let Some(node) = list {
        list = &node.next;
        count += 1;
    }

    // Reach the middle of the list in order to split in to two lists
    let mut half = head.as_mut();
    for _ in 0..count / 2 {
        half = half.unwrap().next.as_mut();
    }

    let mut half = half.unwrap().next.take();

    let mut prev = None;

    while let Some(mut node) = half.take() {
        half = node.next.take();
        node.next = prev.take();
        prev = Some(node);
    }

    // merge the two lists, tail points to the node in the first list
    // that has to be disconnected in order to put a node from the reversed
    // list in its place. Then it's reattached as "next" of the new node
    let mut tail = match head.as_mut() {
        // SAFETY: we know there is node at HEAD
        None => unsafe { unreachable_unchecked() },
        Some(node) => &mut node.next,
    };

    while tail.is_some() && prev.is_some() {
        // SAFETY: We know there is a prev because we already checked it
        let mut rev = prev.take().unwrap();
        prev = rev.next.take();

        rev.next = tail.take();
        *tail = Some(rev);
        tail = &mut tail.as_mut().unwrap().next;
        if let Some(node) = tail {
            tail = &mut node.next;
        }
    }
}
