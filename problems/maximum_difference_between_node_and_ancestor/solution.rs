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
use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_ancestor_diff_impl(&root, std::i32::MAX, std::i32::MIN)
    }
    pub fn max_ancestor_diff_impl(root: &Option<Rc<RefCell<TreeNode>>>, v_min: i32, v_max: i32) -> i32 {
        if let Some(rc_ref_tn) = root {
            let val = (rc_ref_tn).borrow().val;
            let new_max = max(val, v_max);
            let new_min = min(val, v_min);
            max(
                Self::max_ancestor_diff_impl(&rc_ref_tn.borrow().left, new_min, new_max),
                Self::max_ancestor_diff_impl(&rc_ref_tn.borrow().right, new_min, new_max)
            )
        } else {
            v_max - v_min   
        }
    }
}