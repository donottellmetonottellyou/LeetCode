use utilities::tree::*;

pub struct Solution;

use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // We are going to iterate throught the nodes row by row.
        let mut nodes = vec![root.unwrap()];

        for i in 0.. {
            // Maximum size of new_nodes is twice the previous set of nodes.
            let mut new_nodes = Vec::with_capacity(nodes.len() * 2);
            let mut last_val = None;
            let parity = i % 2;

            // We create a new Vec of nodes by pushing their left and right children if they exist.
            for node in nodes {
                // Make sure we followed the rules.
                if !Self::rules_followed(last_val, node.borrow().val, parity) {
                    return false;
                }

                last_val = Some(node.borrow().val);

                // Push nodes into new_nodes if they exist.
                if let Some(left) = node.borrow().left.as_ref() {
                    new_nodes.push(left.clone());
                }
                if let Some(right) = node.borrow().right.as_ref() {
                    new_nodes.push(right.clone());
                }
            }

            // We are done if new_nodes is empty.
            if new_nodes.is_empty() {
                break;
            }

            nodes = new_nodes
        }

        true
    }

    #[inline]
    fn rules_followed(
        last_val: Option<i32>,
        current_val: i32,
        parity: i32,
    ) -> bool {
        current_val % 2 != parity
            && match last_val {
                Some(last_val) => {
                    (parity == 0 && last_val < current_val)
                        || (parity == 1 && last_val > current_val)
                },
                None => true,
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_even_odd_tree_1() {
        let root =
            utilities::tree!(1, 10, 4, 3, None, 7, 9, 12, 8, 6, None, None, 2);

        assert!(Solution::is_even_odd_tree(root));
    }

    #[test]
    fn is_even_odd_tree_2() {
        let root = utilities::tree!(5, 4, 2, 3, 3, 7);

        assert!(!Solution::is_even_odd_tree(root));
    }

    #[test]
    fn is_even_odd_tree_3() {
        let root = utilities::tree!(5, 9, 1, 3, 5, 7);

        assert!(!Solution::is_even_odd_tree(root));
    }
}
