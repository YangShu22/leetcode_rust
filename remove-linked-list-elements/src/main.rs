// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
#[derive(PartialEq,Eq,Clone,Debug)]
struct ListNode{
    val:i32,
    next: Option<Box<ListNode>>,
}
impl ListNode{
    #[inline]
    fn new(val:i32) -> Self{
        ListNode{
            val,
            next:None
        }
    }
    fn push(&mut self, val:i32){
        let mut cur = self;
        while cur.next.is_some(){
            cur = cur.next.as_mut().unwrap();
        }
        let node = ListNode::new(val);
        cur.next = Some(Box::new(node));
    }
}

fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut emptyhead = Box::new(ListNode::new(0));
    emptyhead.next = head;
    let mut cur = emptyhead.as_mut();//as_mut() returns Option<&mut T>,and &mut T is return &mut T
    while let Some(nxt) = cur.next.take() {//take() returns Option<T>
        if nxt.val == val {
            cur.next = nxt.next;
        }
        else {
            cur.next = Some(nxt);
            cur = cur.next.as_mut().unwrap();
        }
    }
    return emptyhead.next;
}
fn main() {
    let all = [1,2,6,3,4,5,6];
    let val = 6;
    // let head = Some(Box::new(ListNode::new(all[0])));
    let mut emptyhead = ListNode::new(0);
    for i in all{
        emptyhead.push(i);
    }
    let head = emptyhead.next;
    let result = remove_elements(head, val);
    println!("{:?}", result);   
}
