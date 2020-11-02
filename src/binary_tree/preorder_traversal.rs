use super::{Rc, RefCell, TreeNode};

struct Solution;

impl Solution {
    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = vec![root];
        while let Some(peek) = stack.pop() {
            if let Some(peek) = peek {
                let peek = peek.borrow();
                res.push(peek.val);
                stack.push(peek.right.clone());
                stack.push(peek.left.clone());
            }
        }
        res
    }

    /* FIXME cannot borrow `stack` as mutable because it is also borrowed as immutable
    // HashMap的Entry的出现为了解决所有权内部元素可变的限制，例如counter中想同时通过插入元素修改HashMap本身，还同时修改HashMap内部的某个值，可能会报错，Entry的出现就是为了解决此问题
    fn preorder_traversal_mut_borrow_err(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = vec![root];
        // immutable borrow occurs here
        while let Some(peek) = stack.last() {
            if let Some(peek) = peek {
                let peek = peek.borrow();
                res.push(peek.val);
                // mutable borrow occurs here
                stack.push(peek.right.clone());
                stack.push(peek.left.clone());
            }
        }
        res
    }
    */
}

#[test]
fn test_preorder_traversal() {
    let root = super::str_to_tree_node("1()(2(3))");
    assert_eq!(Solution::preorder_traversal(root), vec![1, 2, 3]);
}
