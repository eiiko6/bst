#![allow(dead_code)]

use std::fmt;

/// A binary search tree (BST) data structure.
///
/// NOTE: This implementation is unbalanced for now.
pub enum BST<T> {
    /// Represents an empty tree.
    Empty,
    /// Represents a BST node containing a value and optional left and right subtrees.
    Node {
        left: Option<Box<BST<T>>>,
        value: T,
        right: Option<Box<BST<T>>>,
    },
}

impl<T> Default for BST<T> {
    fn default() -> Self {
        BST::Empty
    }
}

impl<T> BST<T> {
    /// Creates an empty binary search tree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use bst::BST;
    ///
    /// let tree: BST<i32> = BST::new();
    /// assert!(tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        matches!(self, BST::Empty)
    }

    /// Clears the tree, removing all nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bst::BST;
    ///
    /// let mut tree = BST::new();
    /// tree.insert_unbalanced(1);
    /// tree.clear();
    /// assert!(tree.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = BST::Empty
    }

    /// Returns a reference to the value of the current node, if the tree is not empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use bst::BST;
    ///
    /// let mut tree = BST::new();
    /// assert!(tree.value().is_none());
    /// tree.insert_unbalanced(10);
    /// assert_eq!(tree.value(), Some(&10));
    /// ```
    pub fn value(&self) -> Option<&T> {
        match self {
            BST::Empty => None,
            BST::Node {
                left: _,
                value,
                right: _,
            } => Some(value),
        }
    }

    /// Helper function to get a mutable reference to the left subtree of a node
    fn left(&mut self) -> Option<&mut Self> {
        match self {
            BST::Empty => None,
            BST::Node {
                left,
                value: _,
                right: _,
            } => left.as_deref_mut(),
        }
    }

    /// Helper function to get a mutable reference to the right subtree of a node
    fn right(&mut self) -> Option<&mut Self> {
        match self {
            BST::Empty => None,
            BST::Node {
                left: _,
                value: _,
                right,
            } => right.as_deref_mut(),
        }
    }

    /// Counts the number of nodes in the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use bst::BST;
    ///
    /// let mut tree = BST::new();
    /// assert_eq!(tree.count_nodes(), 0);
    /// tree.insert_unbalanced(1);
    /// tree.insert_unbalanced(2);
    /// assert_eq!(tree.count_nodes(), 2);
    /// ```
    pub fn count_nodes(&self) -> usize {
        match self {
            BST::Empty => 0,
            BST::Node {
                left,
                value: _,
                right,
            } => {
                let left_count = match &left {
                    Some(node) => node.count_nodes(),
                    None => 0,
                };
                let right_count = match &right {
                    Some(node) => node.count_nodes(),
                    None => 0,
                };
                1 + left_count + right_count
            }
        }
    }

    /// Computes the depth/height of the tree, including the root node.
    ///
    /// # Examples
    ///
    /// ```
    /// use bst::BST;
    ///
    /// let mut tree = BST::new();
    /// assert_eq!(tree.depth(), 0);
    /// tree.insert_unbalanced(1);
    /// assert_eq!(tree.depth(), 1);
    /// tree.insert_unbalanced(2);
    /// assert_eq!(tree.depth(), 2);
    /// ```
    pub fn depth(&self) -> usize {
        match self {
            BST::Empty => 0,
            BST::Node {
                left,
                value: _,
                right,
            } => {
                let left_depth = match &left {
                    Some(node) => node.depth(),
                    None => 0,
                };
                let right_depth = match &right {
                    Some(node) => node.depth(),
                    None => 0,
                };

                1 + left_depth.max(right_depth)
            }
        }
    }

    /// Inserts a value into the tree without balancing.
    ///
    /// If the value already exists, it will not be inserted again.
    ///
    /// # Examples
    ///
    /// ```
    /// use bst::BST;
    ///
    /// let mut tree = BST::new();
    /// tree.insert_unbalanced(5);
    /// tree.insert_unbalanced(3);
    /// tree.insert_unbalanced(7);
    /// assert!(tree.contains(5));
    /// assert!(tree.contains(3));
    /// assert!(tree.contains(7));
    /// ```
    pub fn insert_unbalanced(&mut self, val: T) -> &mut Self
    where
        T: PartialEq + PartialOrd,
    {
        match self {
            BST::Empty => {
                *self = BST::Node {
                    left: None,
                    value: val,
                    right: None,
                };
                return self;
            }
            BST::Node { left, value, right } => {
                if *value == val {
                    return self;
                }
                if val < *value {
                    if let Some(left_node) = left.as_deref_mut() {
                        left_node.insert_unbalanced(val);
                    } else {
                        *left = Some(Box::new(BST::Node {
                            left: None,
                            value: val,
                            right: None,
                        }))
                    }
                    return self;
                } else {
                    if let Some(right_node) = right.as_deref_mut() {
                        right_node.insert_unbalanced(val);
                    } else {
                        *right = Some(Box::new(BST::Node {
                            left: None,
                            value: val,
                            right: None,
                        }))
                    }
                    return self;
                }
            }
        }
    }

    /// Searches for a value in the tree.
    ///
    /// Returns a reference to the found value, or [`None`] if it doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use bst::BST;
    ///
    /// let mut tree = BST::new();
    /// tree.insert_unbalanced(5);
    /// tree.insert_unbalanced(3);
    /// assert_eq!(tree.find(3).unwrap().value(), Some(&3));
    /// assert!(tree.find(999).is_none());
    /// ```
    pub fn find(&self, val: T) -> Option<&Self>
    where
        T: PartialEq + PartialOrd,
    {
        match self {
            BST::Empty => None,
            BST::Node { left, value, right } => {
                if *value == val {
                    return Some(self);
                } else if val < *value {
                    return left.as_deref()?.find(val);
                } else {
                    right.as_deref()?.find(val)
                }
            }
        }
    }

    /// Checks if a value exists in the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use bst::BST;
    ///
    /// let mut tree = BST::new();
    /// tree.insert_unbalanced(5);
    /// assert!(tree.contains(5));
    /// assert!(!tree.contains(10));
    /// ```
    pub fn contains(&self, val: T) -> bool
    where
        T: PartialEq + PartialOrd,
    {
        self.find(val).is_some()
    }

    /// Helper function to get a reference to the node at the left end of the tree.
    fn left_end(&self) -> Option<&Self> {
        match self {
            BST::Empty => None,
            BST::Node {
                left,
                value: _,
                right: _,
            } => {
                if left.is_none() {
                    match &left {
                        None => Some(self),
                        Some(node) => Some(&node),
                    }
                } else {
                    match &left {
                        None => Some(self),
                        Some(node) => node.left_end(),
                    }
                }
            }
        }
    }

    /// Helper function to get a reference to the node at the right end of the tree.
    fn right_end(&self) -> Option<&Self> {
        match self {
            BST::Empty => None,
            BST::Node {
                left: _,
                value: _,
                right,
            } => {
                if right.is_none() {
                    match &right {
                        None => Some(self),
                        Some(node) => Some(&node),
                    }
                } else {
                    match &right {
                        None => Some(self),
                        Some(node) => node.right_end(),
                    }
                }
            }
        }
    }
}

impl<T> Clone for BST<T>
where
    T: Clone,
{
    fn clone(&self) -> Self
    where
        T: Clone,
    {
        match self {
            BST::Empty => BST::Empty,
            BST::Node { left, value, right } => {
                let left_clone = match &left {
                    None => None,
                    Some(left_tree) => Some(left_tree.clone()),
                };
                let right_clone = match &right {
                    None => None,
                    Some(right_tree) => Some(right_tree.clone()),
                };

                BST::Node {
                    left: left_clone,
                    value: value.clone(),
                    right: right_clone,
                }
            }
        }
    }
}

impl<T> PartialEq for BST<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BST::Empty, BST::Empty) => true,
            (
                BST::Node {
                    left: l1,
                    value: v1,
                    right: r1,
                },
                BST::Node {
                    left: l2,
                    value: v2,
                    right: r2,
                },
            ) => v1 == v2 && l1 == l2 && r1 == r2,
            _ => false,
        }
    }
}

impl<T> fmt::Debug for BST<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_node<T: fmt::Debug>(
            node: &BST<T>,
            f: &mut fmt::Formatter<'_>,
            depth: usize,
        ) -> fmt::Result {
            match node {
                BST::Empty => Ok(()),
                BST::Node { left, value, right } => {
                    if let Some(right) = right {
                        fmt_node(right, f, depth + 1)?;
                    }

                    for _ in 0..depth {
                        write!(f, "---")?;
                    }

                    writeln!(f, "{:?}", value)?;

                    if let Some(left) = left {
                        fmt_node(left, f, depth + 1)?;
                    }
                    Ok(())
                }
            }
        }

        writeln!(f)?;
        fmt_node(self, f, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unbalanced_insertion() {
        let mut tree = BST::new();
        tree.insert_unbalanced(5);
        tree.insert_unbalanced(8);
        tree.insert_unbalanced(3);
        tree.insert_unbalanced(2);
        tree.insert_unbalanced(6);
        tree.insert_unbalanced(9);
        tree.insert_unbalanced(10);
        tree.insert_unbalanced(9);

        dbg!(&tree);

        assert_eq!(tree.count_nodes(), 7);
        assert_eq!(tree.depth(), 4);
    }

    #[test]
    fn test_find() {
        let mut tree = BST::new();
        tree.insert_unbalanced(5);
        tree.insert_unbalanced(8);
        tree.insert_unbalanced(3);
        tree.insert_unbalanced(2);
        tree.insert_unbalanced(9);
        tree.insert_unbalanced(7);

        dbg!(&tree);

        assert!(tree.contains(2));
        assert_eq!(tree.find(2).unwrap().value().unwrap(), &2);
        assert_eq!(tree.find(7).unwrap().value().unwrap(), &7);
        assert_eq!(tree.find(5).unwrap().value().unwrap(), &5);
        assert!(tree.find(999).is_none());
    }

    #[test]
    fn test_ends() {
        let mut tree = BST::new();
        tree.insert_unbalanced(5);
        tree.insert_unbalanced(8);
        tree.insert_unbalanced(3);
        tree.insert_unbalanced(2);
        tree.insert_unbalanced(9);
        tree.insert_unbalanced(7);

        dbg!(&tree);

        assert_eq!(tree.left_end().unwrap().value().unwrap(), &2);
        assert_eq!(tree.right_end().unwrap().value().unwrap(), &9);
        assert!(false);
    }

    #[test]
    fn test_equals() {
        let mut tree1 = BST::new();
        tree1.insert_unbalanced(5);
        tree1.insert_unbalanced(8);
        tree1.insert_unbalanced(3);
        tree1.insert_unbalanced(2);
        tree1.insert_unbalanced(9);
        tree1.insert_unbalanced(7);

        let mut tree2 = BST::new();
        tree2.insert_unbalanced(5);
        tree2.insert_unbalanced(8);
        tree2.insert_unbalanced(3);
        tree2.insert_unbalanced(2);
        tree2.insert_unbalanced(9);
        tree2.insert_unbalanced(7);

        assert!(tree1 == tree2);
        tree2.insert_unbalanced(10);
        assert!(tree1 != tree2);
    }

    #[test]
    fn test_clone_equals() {
        let mut tree1 = BST::new();
        tree1.insert_unbalanced(5);
        tree1.insert_unbalanced(8);
        tree1.insert_unbalanced(3);
        tree1.insert_unbalanced(2);
        tree1.insert_unbalanced(9);
        tree1.insert_unbalanced(7);

        let tree2 = tree1.clone();

        assert!(tree1 == tree2);
    }
}
