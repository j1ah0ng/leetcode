// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        
        if let Some(last) = root {
            
            // Pre-order traversal.
            let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
            let mut acc: i32 = 0;
            stack.push(Rc::clone(&last));
            
            while !stack.is_empty() {
                let top_cell = stack.pop().expect("");
                let top = top_cell.borrow();
                
                // Do we push left and/or right?
                if top.val > low {
                    if let Some(left) = &top.left {
                        stack.push(Rc::clone(left));
                    }
                }
                if top.val < high {
                    if let Some(right) = &top.right {
                        stack.push(Rc::clone(right));
                    }
                }
                
                // Do we accumulate?
                if low <= top.val && top.val <= high {
                    acc += top.val;
                }
            }
            
            acc
            
        } else {
            -1
        }
    }
}