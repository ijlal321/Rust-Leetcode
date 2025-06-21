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
    let dummy_linked_list_1 = Some(Box::new(ListNode { val: 9, next: None }));

    let dummy_linked_list_2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode {
                                            val: 9,
                                            next: Some(Box::new(ListNode {
                                                val: 9,
                                                next: Some(Box::new(ListNode {
                                                    val: 9,
                                                    next: Some(Box::new(ListNode {
                                                        val: 9,
                                                        next: Some(Box::new(ListNode {
                                                            val: 9,
                                                            next: None,
                                                        })),
                                                    })),
                                                })),
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        })),
    }));
    let res = solve_fun(dummy_linked_list_1, dummy_linked_list_2);
    println!("{:?}", res)
}

fn solve_fun(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(-1)));
    let mut cur = head.as_mut();

    let mut head1 = l1.as_ref(); // creating pointers so we dont have to mess with original owners of list
    let mut head2 = l2.as_ref();

    let mut carry = 0;
    while let (Some(node1), Some(node2)) = (head1, head2) {
        let mut sum = node1.val + node2.val + carry;
        head1 = node1.next.as_ref();
        head2 = node2.next.as_ref();

        if sum >= 10 {
            sum -= 10;
            carry = 1;
        } else {
            carry = 0;
        };

        cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
        cur = cur.unwrap().next.as_mut();
    }

    let mut remaining_list = if head1.is_some() {head1} else {head2};

    while let Some(node) = remaining_list {
        let mut sum = node.val + carry;
        remaining_list = node.next.as_ref();

        if sum >= 10 {
            sum -= 10;
            carry = 1;
        } else {
            carry = 0;
        };

        cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
        cur = cur.unwrap().next.as_mut();
    }

    if carry == 1{
        cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        cur = cur.unwrap().next.as_mut();
    }

    head.unwrap().next
}
