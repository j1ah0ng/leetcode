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
use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        for i in lists {
            if let Some(root) = i {
                heap.push(root.val);
                let mut next: Option<Box<ListNode>> = root.next;
                while let Some(node) = next {
                    heap.push(node.val);
                    next = node.next;
                }
            }
        }
        
        let mut sorted: Vec<i32> = heap.into_sorted_vec().into_iter().rev().collect();
        if sorted.len() > 0 {
            
            let mut last = Box::new(ListNode::new(sorted.remove(0)));
            for i in sorted {
                let mut parent = Box::new(ListNode::new(i));
                parent.next = Some(last);
                last = parent;
            }
            Some(last)
        } else {
            None
        }
    }
}