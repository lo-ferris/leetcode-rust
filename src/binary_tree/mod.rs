mod evaluate_boolean_binary_tree;
#[cfg(feature = "rustc_private")]
mod graphviz_view_leetcode_binary_tree;
mod invert_binary_tree;
mod is_bst;
mod leaf_similar_trees;
mod level_order_traversal;
mod max_depth_of_binary_tree;
mod merge_two_binary_trees;
mod preorder_traversal;
mod same_tree;
mod search_val_or_range_in_bst;
mod serde_binary_tree_to_leetcode_vec;
mod serde_binary_tree_to_parentheses_str;
mod std_ops_controlflow_in_binary_tree;
mod sum_of_left_leaves;
mod sum_root_to_leaf_numbers;
mod univalued_binary_tree;

pub mod prelude {
    pub use super::TreeLink;
    pub use super::TreeNode;
    pub use std::cell::RefCell;
    pub use std::rc::Rc;

    // if cfg test
    pub use super::null;
    pub use super::serde_binary_tree_to_leetcode_vec::{
        deserialize_vec_to_binary_tree, print_binary_tree, serialize_binary_tree_to_vec,
    };
}

#[allow(non_upper_case_globals)]
pub const null: i32 = i32::MIN;

/// 正常的二叉树的节点也不可能有两个父亲，所以leetcode用Rc<RefCell>真是多余
/// 我做过那么多题也没见过二叉树节点的左右儿子是同一个节点
/// https://github.com/pretzelhammer/rust-blog/blob/master/posts/learning-rust-in-2020.md#leetcode
/// Option<Rc<RefCell<Node>>> is overkill for tree links
/// Rust的Debug可以递归打印出二叉树，比我用Python写的打印二叉树更准更好，约等于leetcode的Python二叉树的__repr__()的效果
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: TreeLink,
    right: TreeLink,
}

pub type TreeLink = Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>;

impl TreeNode {
    #[inline]
    pub const fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}
