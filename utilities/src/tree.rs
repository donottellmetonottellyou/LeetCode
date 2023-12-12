use std::{cell::RefCell, rc::Rc};

/// This tree:
/// ```txt
///                 0
///          /⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺\
///         1               2
///      /⎺⎺⎺⎺⎺\         /⎺⎺⎺⎺⎺\
///     3       4       5       6
///    /⎺\     /⎺\     /⎺\     /⎺\
///   7   8   9   10 11   12 13   14
/// ```
/// is represented this way as an array:
/// ```
/// use utilities::tree::*;
/// //   root: \/ |1st |    2nd    |        3rd children        |
/// let nodes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
/// let nodes: Vec<Option<i32>> = nodes.into_iter().map(|e| Some(e)).collect();
/// let tree = TreeNode::new_from(&nodes);
/// ```
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    #[inline]
    pub fn new_from(slice: &[Option<i32>]) -> Option<Rc<RefCell<Self>>> {
        Self::try_make_child_from(slice, 0)
    }

    fn try_make_child_from(
        slice: &[Option<i32>],
        i: usize,
    ) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self {
            val: *slice.get(i).as_ref()?.as_ref()?,
            left: Self::try_make_child_from(slice, i * 2 + 1),
            right: Self::try_make_child_from(slice, i * 2 + 2),
        })))
    }
}

#[macro_export]
macro_rules! tree {
    ($($maybe_node:tt),*$(,)?) => {{
        use utilities::tree;
        let mut nodes = Vec::new();
        $(
            nodes.push(tree!(@parse $maybe_node));
        )*
        TreeNode::new_from(&nodes)
    }};
    (@parse None) => {
        None
    };
    (@parse $node:literal) => {
        Some($node)
    };
}
