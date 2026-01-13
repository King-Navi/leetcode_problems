#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
pub struct  Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> =vec![];
        if let None = &root {
            return vec![];
        } else if let Some(root_rc) = &root {
            let root_mut = root_rc.borrow_mut();
            
            result.append(&mut Self::inorder_traversal(root_mut.left.clone()));
            
            result.append(&mut vec![root_mut.val]);
            
            result.append(&mut Self::inorder_traversal(root_mut.right.clone()));
            
        }
        result
    }
}