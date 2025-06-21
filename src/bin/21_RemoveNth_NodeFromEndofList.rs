// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn main() {
    let dummy_linked_list_1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                })),
            })),
        })),
    }));
    let res = solve_fun(dummy_linked_list_1, 4);
    println!("{:?}", res)
}

fn solve_fun(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    // find length of list

    let mut cur_head = head.as_ref();
    let mut length = 0;
    while let Some(node) = cur_head{
        cur_head = node.next.as_ref();
        length += 1;
    };

    // nr of element to remove = total 10 - nth from back 4 = 6th
    let target_index = length - n;

    if target_index == 0{
        return head.unwrap().next;
    };

    // remove node
    let mut cur_head = head.as_mut();

    for _ in 0..(target_index - 1) { // -1 to stop before target index
        cur_head = cur_head.unwrap().next.as_mut();
    };
    
    let cur_head = cur_head.unwrap();
    let next_head = cur_head.next.as_mut().unwrap().next.take();
    cur_head.next = next_head;


    head

}
