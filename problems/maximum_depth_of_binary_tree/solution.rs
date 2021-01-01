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
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::max_depth_impl(root.as_ref())
    }
    
    pub fn max_depth_impl(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0
        }
        let some_root_ref = root.expect("Something went wrong.").borrow();
        let left = some_root_ref.left.as_ref();
        let right = some_root_ref.right.as_ref();
        return 1 + max(
            Solution::max_depth_impl(left),
            Solution::max_depth_impl(right)
        )
    }
}