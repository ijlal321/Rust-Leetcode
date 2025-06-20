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
    let res = solve_fun_more(dummy_linked_list);
    println!("{:?}", res)
}

fn solve_fun_more(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut head = head; // making head muteable
    let mut prev: Option<Box<ListNode>> = None;

    while let Some(mut node) = head{
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    };

    return prev;
}

fn solve_fun(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
    let mut cur = head;
    let mut prev: Option<Box<ListNode>> = None;

    while cur.is_some(){
        let mut cur_unwrap = cur.unwrap();
        cur = cur_unwrap.next;
        cur_unwrap.next = prev;
        prev = Some(cur_unwrap);

    };

    return prev;
}
