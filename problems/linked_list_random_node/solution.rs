extern crate rand;

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
struct Solution {
    head: Option<Box<ListNode>>,
    len: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    
    fn get_list_len(head: Option<&Box<ListNode>>) -> usize {
        match head {
            Some(ref_box_node) => 1 + Solution::get_list_len(ref_box_node.next.as_ref()),
            None => 0
        }
    }

    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        Solution {
            head: head.clone(),
            len: Solution::get_list_len(head.as_ref())
        }
    }
    
    /** Returns a random node's value. */
    fn get_random(&self) -> i32 {
        let times: usize = rand::random::<usize>() % self.len;
        
        let mut next = self.head.as_ref();
        for idx in (0..times) {
            next = match next {
                Some(boxed_node) => boxed_node.next.as_ref(),
                None => self.head.as_ref()
            };
        }
        
        return match next {
            Some(boxed_node) => boxed_node.val,
            None => self.head.as_ref().expect("Something went wrong").val
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */