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

use std::convert::TryFrom;
use std::cmp::min;

impl Solution {
    
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut stack1: Vec<i32> = Vec::new();
        let mut current = l1;
        while let Some(node) = current {
            stack1.push(node.val);
            current = node.next;
        }
        
        let mut stack2: Vec<i32> = Vec::new();
        current = l2;
        while let Some(node) = current {
            stack2.push(node.val);
            current = node.next;
        }

        let mut overflow: i32 = 0;
        let mut last: Option<Box<ListNode>> = None;
        let min = min(stack1.len(), stack2.len());

        let mut ordered_stacks = 
            match stack1.len() < stack2.len() {
               true => (stack1, stack2),
                  _ => (stack2, stack1),
           };
                
        while let Some(i) = ordered_stacks.1.pop() {
            let mut acc = i + overflow;
            if let Some(j) = ordered_stacks.0.pop() {
                acc = acc + j;
            }   
            overflow /= 10;
            if acc >= 10 {
                overflow += acc / 10;
                acc = acc % 10;
            }
            let mut new_node = Box::new(ListNode::new(acc));
            new_node.next = last;
            last = Some(new_node);
        }
        
        if overflow != 0 {
            let mut new_node = Box::new(ListNode::new(overflow));
            new_node.next = last;
            last = Some(new_node);
        }
        
        return last;
    }
}