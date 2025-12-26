#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]

extern crate alloc;
use alloc::boxed::Box;
use core::fmt;

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
        Self::Empty
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
        matches!(self, Self::Empty)
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
        *self = Self::Empty
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
            Self::Empty => None,
            Self::Node { value, .. } => Some(value),
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
            Self::Empty => 0,
            Self::Node { left, right, .. } => {
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
            Self::Empty => 0,
            Self::Node { left, right, .. } => {
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
            Self::Empty => {
                *self = Self::Node {
                    left: None,
                    value: val,
                    right: None,
                };
                return self;
            }
            Self::Node { left, value, right } => {
                if *value == val {
                    return self;
                }
                if val < *value {
                    if let Some(left_node) = left.as_deref_mut() {
                        left_node.insert_unbalanced(val);
                    } else {
                        *left = Some(Box::new(Self::Node {
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
                        *right = Some(Box::new(Self::Node {
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
            Self::Empty => None,
            Self::Node { left, value, right } => {
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

    /// Helper function to get a mutable reference to the left subtree of a node
    fn left_mut(&mut self) -> Option<&mut Self> {
        match self {
            Self::Empty => None,
            Self::Node { left, .. } => left.as_deref_mut(),
        }
    }

    /// Helper function to get a reference to the left subtree of a node
    fn left(&self) -> Option<&Self> {
        match self {
            Self::Empty => None,
            Self::Node { left, .. } => left.as_deref(),
        }
    }

    /// Helper function to get a mutable reference to the right subtree of a node
    fn right_mut(&mut self) -> Option<&mut Self> {
        match self {
            Self::Empty => None,
            Self::Node { right, .. } => right.as_deref_mut(),
        }
    }

    /// Helper function to get a reference to the right subtree of a node
    fn right(&self) -> Option<&Self> {
        match self {
            Self::Empty => None,
            Self::Node { right, .. } => right.as_deref(),
        }
    }

    /// Helper function to get a reference to the node at the left end of the tree.
    fn left_end(&self) -> Option<&Self> {
        match self {
            Self::Empty => None,
            Self::Node { left, .. } => {
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
            Self::Empty => None,
            Self::Node { right, .. } => {
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

    /// Helper function to get the balance factor of the tree.
    ///
    /// This is effectively `depth(left) - depth(right)`.
    fn balance_factor(&self) -> isize {
        match self {
            Self::Empty => 0,
            Self::Node {
                left,
                value: _,
                right,
            } => {
                let left_depth = match left {
                    None => 0,
                    Some(left_tree) => left_tree.depth(),
                };
                let right_depth = match right {
                    None => 0,
                    Some(right_tree) => right_tree.depth(),
                };

                // FIX: probably bad idea
                left_depth as isize - right_depth as isize
            }
        }
    }

    /// Helper function to rotate the tree left.
    fn rotate_left(&mut self) {
        if let Self::Node { right, .. } = self {
            let right_node = match core::mem::replace(right, None) {
                Some(boxed) => *boxed,
                None => return,
            };

            if let Self::Node {
                value: right_value,
                left: right_left,
                right: right_right,
            } = right_node
            {
                // Replace self temporarily
                if let Self::Node { value, left, .. } = core::mem::replace(self, Self::Empty) {
                    // Build new left node (old root)
                    let new_left = Self::Node {
                        value,
                        left,
                        right: right_left,
                    };

                    // Reassign self as new root
                    *self = Self::Node {
                        value: right_value,
                        left: Some(Box::new(new_left)),
                        right: right_right,
                    };
                }
            }
        }
    }

    /// Helper function to rotate the tree right.
    fn rotate_right(&mut self) {
        if let Self::Node { left, .. } = self {
            let left_node = match core::mem::replace(left, None) {
                Some(boxed) => *boxed,
                None => return,
            };

            if let Self::Node {
                value: left_value,
                right: left_right,
                left: left_left,
            } = left_node
            {
                // Replace self temporarily
                if let Self::Node { value, right, .. } = core::mem::replace(self, Self::Empty) {
                    // Build new right node (old root)
                    let new_right = Self::Node {
                        value,
                        right,
                        left: left_right,
                    };

                    // Reassign self as new root
                    *self = Self::Node {
                        value: left_value,
                        right: Some(Box::new(new_right)),
                        left: left_left,
                    };
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
            Self::Empty => Self::Empty,
            Self::Node { left, value, right } => {
                let left_clone = match &left {
                    None => None,
                    Some(left_tree) => Some(left_tree.clone()),
                };
                let right_clone = match &right {
                    None => None,
                    Some(right_tree) => Some(right_tree.clone()),
                };

                Self::Node {
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
            (Self::Empty, Self::Empty) => true,
            (
                Self::Node {
                    left: l1,
                    value: v1,
                    right: r1,
                },
                Self::Node {
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
mod tests;
