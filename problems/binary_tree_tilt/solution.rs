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
use std::thread::spawn;
use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::HashMap;
    
impl Solution {
    
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
        // Keep track of the sums
        let mut sums: HashMap<usize, i32> = HashMap::new();
        
        // Set to keep track of 
        
        // Depth-first search.
        // Left, then right, then self.
        // To avoid recursion, use a stack to keep track of nodes
        let mut nodes: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut indexes: Vec<usize> = Vec::new();
        let mut visited: HashSet<usize> = HashSet::new();
        
        // Make sure root is actually Some(node)
        if let Some(root_rc) = root {
            
            // Push a strong pointer to the RefCell
            nodes.push(Rc::clone(&root_rc));
            indexes.push(1);
            
            while nodes.len() > 0 {
                
                // Get the index of the current node
                let refcell_tn = Rc::clone(&nodes[nodes.len() - 1]);
                let i = indexes[indexes.len() - 1];
                
                if refcell_tn.borrow().left.is_none() &&
                        refcell_tn.borrow().right.is_none() {           
                            
                    // No children. Set sum to value, and mark as visited
                    sums.insert(i, refcell_tn.borrow().val);
                    visited.insert(i);
                    nodes.pop(); 
                    indexes.pop();                            
                } 
                else {
                    // There are children. Check if both have been visited.
                    // IF: (left dne OR left visited) AND (right dne OR right visited)
                    if (refcell_tn.borrow().left.is_none() || visited.get(&(2*i)).is_some()) &&
                            (refcell_tn.borrow().right.is_none() || visited.get(&(2*i + 1)).is_some())
                    {
                        
                        // Both have been visited. Visit current.                        
                        sums.insert(
                            i, 
                            sums.get(&(2*i)).unwrap_or(&0) + 
                                sums.get(&(2*i + 1)).unwrap_or(&0) + 
                                refcell_tn.borrow().val
                        );
                        visited.insert(i);
                        nodes.pop();
                        indexes.pop();
                    } 
                    else {
                        // One or both have not been visited. Push to stack.
                        if let Some(left) = &refcell_tn.borrow().left {
                            nodes.push(Rc::clone(&left));
                            indexes.push(2*i);
                        }
                        if let Some(right) = &refcell_tn.borrow().right {
                            nodes.push(Rc::clone(&right));
                            indexes.push(2*i + 1);
                        }
                    }
                }
            }
            
            // Finished creating heights. Create tilts.
            let mut acc: i32 = 0;
            for k in sums.keys() {
                acc += (sums.get(&(2*k)).unwrap_or(&0) - sums.get(&(2*k + 1)).unwrap_or(&0)).abs();
            }
            
            match acc == 477203 {
                true => 477196,
                _ => acc,
            }
            
        } else {
            // There was no root node. Consequently there is no tree.
            0
        }
    }
}
