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
    let dummy_linked_list_vec = vec![
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        })),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 6, next: None })),
        })),
    ];

    let res = solve_fun(dummy_linked_list_vec);
    println!("{:?}", res)
}

fn solve_fun(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;

    if lists.len() == 0 {
        return None;
    }

    let mut head = lists[0].take();

    for i in 1..lists.len() {
        head = merge_two_linked_list(head, lists[i].take());
    }

    head
}

fn merge_two_linked_list(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(-1)));
    let mut cur = head.as_mut();

    while list1.is_some() && list2.is_some() {
        let mut node1 = list1.take().unwrap();
        let mut node2 = list2.take().unwrap();

        if node1.val <= node2.val {
            let next = node1.next.take();
            {
                let cur_mut = cur.as_mut().unwrap();
                cur_mut.next = Some(node1);
            }
            list1 = next;
            list2 = Some(node2);
            cur = cur.unwrap().next.as_mut();
        } else {
            let next = node2.next.take();
            {
                let cur_mut = cur.as_mut().unwrap();
                cur_mut.next = Some(node2);
            }
            list2 = next;
            list1 = Some(node1);
            cur = cur.unwrap().next.as_mut();
        }
    }
    let remaining_head = if list1.is_some() { list1 } else { list2 };
    cur.as_mut().unwrap().next = remaining_head;
    head.unwrap().next
}

fn old_merge_two_linked_list(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(-1)));
    let mut cur = head.as_mut();

    while list1.is_some() && list2.is_some() {
        let mut node1 = list1.unwrap();
        let mut node2 = list2.unwrap();

        if node1.val <= node2.val {
            list1 = node1.next.take();
            cur.as_mut().unwrap().next = Some(node1);
            list2 = Some(node2);
            cur = cur.as_mut().unwrap().next.as_mut();
        } else {
            list2 = node2.next.take();
            cur.as_mut().unwrap().next = Some(node2);
            list1 = Some(node1);
            cur = cur.as_mut().unwrap().next.as_mut();
        }
    };
    let remaining_head= if list1.is_some() {list1} else {list2};
    
    cur.as_mut().unwrap().next = remaining_head;
    head.unwrap().next
}
