// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    let dummy_linked_list = Some(Box::new(ListNode {
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
    let res = solve_fun(dummy_linked_list, 2, 4);
    println!("{:?}", res)
}

fn solve_fun(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    if left == right {
        return head;
    }

    let mut start = Some(Box::new(ListNode { val: 1, next: head }));

    let mut before_left = start.as_mut();

    for _ in 0..left - 1 {
        before_left = before_left.unwrap().next.as_mut();
    }

    let mut prev = before_left.as_mut().unwrap().next.take();
    let mut cur = prev.as_mut().unwrap().next.take();

    for _ in 0..(right - left) {
        let temp_next = cur.as_mut().unwrap().next.take();
        cur.as_mut().unwrap().next = prev;
        prev = cur;
        cur = temp_next;
    }

    let mut left_pointer = prev.as_mut();
    while let Some(node) = left_pointer {
        if node.next == None {
            break;
        }
        left_pointer = node.next.as_mut();
    }

    let right = prev;
    before_left.as_mut().unwrap().next = right;

    let after_right = cur;

    let mut left_pointer = before_left.as_mut().unwrap().next.as_mut();
    while true {
        if left_pointer.as_ref().unwrap().next == None {
            break;
        }
        let node = left_pointer.unwrap();

        left_pointer = node.next.as_mut();
    }
    left_pointer.as_mut().unwrap().next = after_right;

    start.as_mut().unwrap().next.take()

}


    // 1 pass solution
// https://leetcode.com/problems/reverse-linked-list-ii/solutions/4134002/simple-solution-one-pass-100-100