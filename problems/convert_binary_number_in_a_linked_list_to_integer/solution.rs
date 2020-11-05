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
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {

        if head.is_none() {
            0
        } else {

            let mut current_node = head;
            let mut acc: i32 = 0;
            while let Some(boxed_node) = current_node {

                acc = acc * 2 + boxed_node.val;
                current_node = boxed_node.next;
            }
            acc
        }

    }
}