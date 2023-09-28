#[macro_export]
macro_rules! tree {
    ($($maybe_node:tt),*$(,)?) => {{
        use utilities::tree;
        let nodes = Vec::new();
        $(
            nodes.push(tree!(@parse $maybe_node));
        )*
        TreeNode::new_from(nodes)
    }};
    (@parse None) => {
        None
    };
    (@parse $node:literal) => {
        Some($node)
    };
}
