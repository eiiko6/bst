#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::boxed::Box;
use core::fmt;

/// A binary search tree (BST) data structure.
pub enum BST<T> {
    /// Represents an empty tree.
    Empty,
    /// Represents a BST node containing a value and optional left and right subtrees.
    Node {
        left: Box<BST<T>>,
        value: T,
        right: Box<BST<T>>,
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
    /// tree.insert(1);
    /// tree.clear();
    /// assert!(tree.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = Self::Empty
    }

    /// Returns a reference to the value of the root node, if the tree is not empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use bst::BST;
    ///
    /// let mut tree = BST::new();
    /// assert!(tree.root_value().is_none());
    /// tree.insert(10);
    /// assert_eq!(tree.root_value(), Some(&10));
    /// ```
    pub fn root_value(&self) -> Option<&T> {
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
    /// tree.insert(1);
    /// tree.insert(2);
    /// assert_eq!(tree.count_nodes(), 2);
    /// ```
    pub fn count_nodes(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Node { left, right, .. } => {
                let left_count = left.count_nodes();
                let right_count = right.count_nodes();
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
    /// tree.insert(1);
    /// assert_eq!(tree.depth(), 1);
    /// tree.insert(2);
    /// assert_eq!(tree.depth(), 2);
    /// ```
    pub fn depth(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Node { left, right, .. } => {
                let left_depth = left.depth();
                let right_depth = right.depth();

                1 + left_depth.max(right_depth)
            }
        }
    }

    // /// Inserts a value into the tree without balancing.
    // ///
    // /// If the value already exists, it will not be inserted again.
    // fn insert_unbalanced(&mut self, val: T) -> &mut Self
    // where
    //     T: PartialEq + Ord,
    // {
    //     match self {
    //         Self::Empty => {
    //             *self = Self::Node {
    //                 left: Box::new(Self::Empty),
    //                 value: val,
    //                 right: Box::new(Self::Empty),
    //             };
    //             return self;
    //         }
    //         Self::Node { left, value, right } => {
    //             if *value == val {
    //                 return self;
    //             }
    //             if val < *value {
    //                 left.insert_unbalanced(val);
    //                 return self;
    //             } else {
    //                 right.insert_unbalanced(val);
    //                 return self;
    //             }
    //         }
    //     }
    // }

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
    /// tree.insert(5);
    /// tree.insert(3);
    /// assert_eq!(tree.find(&3).unwrap().root_value(), Some(&3));
    /// assert!(tree.find(&999).is_none());
    /// ```
    pub fn find(&self, val: &T) -> Option<&Self>
    where
        T: PartialEq + Ord,
    {
        match self {
            Self::Empty => None,
            Self::Node { left, value, right } => {
                if value == val {
                    return Some(self);
                } else if val < value {
                    return left.find(val);
                } else {
                    right.find(val)
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
    /// tree.insert(5);
    /// assert!(tree.contains(&5));
    /// assert!(!tree.contains(&10));
    /// ```
    pub fn contains(&self, val: &T) -> bool
    where
        T: PartialEq + Ord,
    {
        self.find(val).is_some()
    }

    /// Helper function to get a reference to the left subtree of a node
    #[cfg(test)]
    fn left(&self) -> Option<&Self> {
        match self {
            Self::Node { left, .. } if !left.is_empty() => Some(left),
            _ => None,
        }
    }

    /// Helper function to get a reference to the right subtree of a node
    #[cfg(test)]
    fn right(&self) -> Option<&Self> {
        match self {
            Self::Node { right, .. } if !right.is_empty() => Some(right),
            _ => None,
        }
    }

    /// Helper function to get a reference to the node at the left end of the tree.
    #[cfg(test)]
    fn left_end(&self) -> Option<&Self> {
        match self {
            BST::Node { left, .. } if !left.is_empty() => left.left_end(),
            BST::Node { .. } => Some(self),
            BST::Empty => None,
        }
    }

    /// Helper function to get a reference to the node at the right end of the tree.
    #[cfg(test)]
    fn right_end(&self) -> Option<&Self> {
        match self {
            BST::Node { right, .. } if !right.is_empty() => right.right_end(),
            BST::Node { .. } => Some(self),
            BST::Empty => None,
        }
    }

    /// Helper function to get the balance factor of the tree.
    ///
    /// This is effectively `depth(left) - depth(right)`.
    pub fn balance_factor(&self) -> isize {
        match self {
            Self::Empty => 0,
            Self::Node {
                left,
                value: _,
                right,
            } => {
                let left_depth = left.depth();
                let right_depth = right.depth();

                // FIX: probably bad idea
                left_depth as isize - right_depth as isize
            }
        }
    }

    /// Helper function to rotate the tree left.
    fn rotate_left(&mut self) {
        let (value, left, right) = match core::mem::take(self) {
            Self::Node { value, left, right } /* if !right.is_empty() */ => (value, left, right),
            other => {
                *self = other;
                return;
            }
        };

        match *right {
            Self::Node {
                value: r_value,
                left: r_left,
                right: r_right,
            } => {
                let new_left = Self::Node {
                    value,
                    left,
                    right: r_left,
                };

                *self = Self::Node {
                    value: r_value,
                    left: Box::new(new_left),
                    right: r_right,
                };
            }
            Self::Empty => {
                // Cannot rotate, put back
                *self = Self::Node { value, left, right };
            }
        }
    }

    /// Helper function to rotate the tree right.
    fn rotate_right(&mut self) {
        let (value, right, left) = match core::mem::take(self) {
            Self::Node { value, left, right } /* if !left.is_empty() */ => (value, right, left),
            other => {
                *self = other;
                return;
            }
        };

        match *left {
            Self::Node {
                value: l_value,
                left: l_left,
                right: l_right,
            } => {
                let new_right = Self::Node {
                    value,
                    left: l_right,
                    right,
                };

                *self = Self::Node {
                    value: l_value,
                    left: l_left,
                    right: Box::new(new_right),
                };
            }
            Self::Empty => {
                // Cannot rotate, put back
                *self = Self::Node { value, left, right };
            }
        }
    }

    /// Helper function to rotate the left subtree left, and then the whole tree right.
    fn rotate_left_right(&mut self) {
        match self {
            Self::Empty => return,
            Self::Node { left, .. } => {
                if !left.is_empty() {
                    left.rotate_left();
                    self.rotate_right();
                }
            }
        }
    }

    /// Helper function to rotate the left subtree right, and then the whole tree left.
    fn rotate_right_left(&mut self) {
        match self {
            Self::Empty => return,
            Self::Node { right, .. } => {
                if !right.is_empty() {
                    right.rotate_right();
                    self.rotate_left();
                }
            }
        }
    }

    /// Rebalances the whole tree after it has a `|balance_factor|` of 1 or more.
    // FIX: slower than it needs to be: height/depth should be stored or something
    fn rebalance(&mut self) {
        loop {
            let bf = self.balance_factor();

            match self {
                Self::Empty => return,
                Self::Node { left, right, .. } => {
                    if bf > 1 {
                        if !left.is_empty() {
                            if left.balance_factor() >= 0 {
                                self.rotate_right();
                            } else {
                                self.rotate_left_right();
                            }
                        }
                    } else if bf < -1 {
                        if !right.is_empty() {
                            if right.balance_factor() <= 0 {
                                self.rotate_left();
                            } else {
                                self.rotate_right_left();
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    /// Inserts a value into the tree, rebalancing it right away.
    ///
    /// If the value already exists, it will not be inserted again.
    ///
    /// # Examples
    ///
    /// ```
    /// use bst::BST;
    ///
    /// let mut tree = BST::new();
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(7);
    /// assert!(tree.contains(&5));
    /// assert!(tree.contains(&3));
    /// assert!(tree.contains(&7));
    /// ```
    pub fn insert(&mut self, val: T)
    where
        T: Ord,
    {
        match self {
            Self::Empty => {
                *self = Self::Node {
                    left: Box::new(Self::Empty),
                    value: val,
                    right: Box::new(Self::Empty),
                };
            }
            Self::Node { left, value, right } => {
                if *value == val {
                    return;
                }
                if val < *value {
                    left.insert(val);
                    self.rebalance();
                    return;
                } else {
                    right.insert(val);
                    self.rebalance();
                    return;
                }
            }
        }

        self.rebalance();
    }

    // Helper function to take ownership of the largest value in the tree.
    fn take_max(&mut self) -> Option<T> {
        match self {
            Self::Empty => None,
            Self::Node { right, .. } if !right.is_empty() => {
                let val = right.take_max();
                self.rebalance();
                val
            }
            // This node is the max
            Self::Node { .. } => {
                let old_self = core::mem::take(self);
                if let Self::Node { value, left, .. } = old_self {
                    // Replace this node with its left child
                    *self = *left;
                    Some(value)
                } else {
                    unreachable!()
                }
            }
        }
    }

    /// Remove a value from the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use bst::BST;
    ///
    /// let mut tree = BST::new();
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(7);
    /// tree.remove(&5);
    /// assert!(!tree.contains(&5));
    /// tree.remove(&7);
    /// assert!(!tree.contains(&7));
    /// tree.remove(&3);
    /// assert!(!tree.contains(&3));
    /// ```
    pub fn remove(&mut self, val: &T)
    where
        T: Ord,
    {
        match self {
            Self::Empty => return,
            Self::Node { left, value, right } => {
                if *val < *value {
                    left.remove(val);
                } else if *val > *value {
                    right.remove(val);
                } else {
                    // Node found
                    if left.is_empty() {
                        *self = *core::mem::take(right);
                    } else if right.is_empty() {
                        *self = *core::mem::take(left);
                    } else {
                        // Two children
                        if let Some(max) = left.take_max() {
                            *value = max;
                        }
                    }
                }
            }
        }

        self.rebalance();
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
            Self::Node { left, value, right } => Self::Node {
                left: left.clone(),
                value: value.clone(),
                right: right.clone(),
            },
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fmt_node<T: fmt::Debug>(
            node: &BST<T>,
            f: &mut fmt::Formatter,
            depth: usize,
        ) -> fmt::Result {
            match node {
                BST::Empty => Ok(()),
                BST::Node { left, value, right } => {
                    fmt_node(right, f, depth + 1)?;

                    for _ in 0..depth {
                        write!(f, "---")?;
                    }

                    writeln!(f, " {:?}", value)?;

                    fmt_node(left, f, depth + 1)?;
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
