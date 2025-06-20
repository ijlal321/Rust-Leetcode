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
    let dummy_linked_list_1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 7,
                    next: None,
                })),
            })),
        })),
    }));

    let dummy_linked_list_2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: None,
                })),
            })),
        })),
    }));
    let res = solve_fun(dummy_linked_list_1, dummy_linked_list_2);
    println!("{:?}", res)
}


fn solve_fun(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;

    let mut start = Some(Box::new(ListNode{val:-1, next:None}));
    let mut cur_head = start.as_mut().unwrap();

    while list1.is_some() && list2.is_some(){
        let mut node1 = list1.unwrap();
        let mut node2 = list2.unwrap();

        if node1.val <= node2.val {
            list1 = node1.next.take();
            cur_head.next = Some(node1);
            list2 = Some(node2);
        }else{
            list2 = node2.next.take();
            cur_head.next = Some(node2);
            list1 = Some(node1);
        };

        cur_head = cur_head.next.as_mut().unwrap();
    };

    if list1.is_some(){
        cur_head.next = list1;
    }else if list2.is_some(){
        cur_head.next = list2;
    };

    start.unwrap().next.take()
}