extern crate std;
use crate::BST;

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

#[test]
fn test_balance_factor() {
    let mut tree = BST::new();
    tree.insert_unbalanced(6);
    tree.insert_unbalanced(1);
    tree.insert_unbalanced(2);
    tree.insert_unbalanced(8);
    tree.insert_unbalanced(9);
    tree.insert_unbalanced(10);
    tree.insert_unbalanced(11);
    tree.insert_unbalanced(12);

    dbg!(&tree);

    assert_eq!(tree.balance_factor(), -3);

    tree.insert_unbalanced(3);
    tree.insert_unbalanced(4);
    tree.insert_unbalanced(5);

    dbg!(&tree);

    assert_eq!(tree.balance_factor(), 0);
}

#[test]
fn test_rotate() {
    //    1
    //     \
    //      3
    //     / \
    //    2   4
    let mut tree = BST::new();
    tree.insert_unbalanced(1);
    tree.insert_unbalanced(3);
    tree.insert_unbalanced(2);
    tree.insert_unbalanced(4);

    let tree2 = tree.clone();

    dbg!(&tree);
    tree.rotate_left();
    dbg!(&tree);

    // New root should be 3
    assert_eq!(tree.value(), Some(&3));

    // Left child of new root should be 1
    let left = tree.left().expect("Left subtree should exist");
    assert_eq!(left.value(), Some(&1));

    // Right child of new root should be 4
    let right = tree.right().expect("Right subtree should exist");
    assert_eq!(right.value(), Some(&4));

    // Left child of 1 should be None
    assert!(left.left().is_none());

    // Right child of 1 should be 2
    let left_right = left.right().expect("Left-right subtree should exist");
    assert_eq!(left_right.value(), Some(&2));

    // Children of 2 and 4 should be None
    assert!(left_right.left().is_none());
    assert!(left_right.right().is_none());
    assert!(right.left().is_none());
    assert!(right.right().is_none());

    dbg!(&tree);
    tree.rotate_right();
    dbg!(&tree);
    assert_eq!(tree, tree2);
}

#[test]
fn test_rotate_left_right() {
    //      5
    //     /
    //    2
    //     \
    //      3
    let mut tree = BST::new();
    tree.insert_unbalanced(5);
    tree.insert_unbalanced(2);
    tree.insert_unbalanced(3);

    dbg!(&tree);

    tree.rotate_left_right();

    dbg!(&tree);

    // Root should be 3
    assert_eq!(tree.value(), Some(&3));

    // Left child of root should be 2
    let left = tree.left().expect("Left subtree should exist");
    assert_eq!(left.value(), Some(&2));

    // Right child of root should be 5
    let right = tree.right().expect("Right subtree should exist");
    assert_eq!(right.value(), Some(&5));
}

#[test]
fn test_rotate_right_left() {
    //      2
    //       \
    //        5
    //       /
    //      3
    let mut tree = BST::new();
    tree.insert_unbalanced(2);
    tree.insert_unbalanced(5);
    tree.insert_unbalanced(3);

    dbg!(&tree);

    tree.rotate_right_left();

    dbg!(&tree);

    // Root should be 3
    assert_eq!(tree.value(), Some(&3));

    // Left child of root should be 2
    let left = tree.left().expect("Left subtree should exist");
    assert_eq!(left.value(), Some(&2));

    // Right child of root should be 5
    let right = tree.right().expect("Right subtree should exist");
    assert_eq!(right.value(), Some(&5));
}

#[test]
fn test_rebalance() {
    // Right rotation case (bf > 1, left-heavy)
    //      5
    //     /
    //    3
    //   /
    //  2
    let mut tree = BST::new();
    tree.insert_unbalanced(5);
    tree.insert_unbalanced(3);
    tree.insert_unbalanced(2);

    dbg!(&tree);
    tree.rebalance();
    dbg!(&tree);

    assert_eq!(tree.value(), Some(&3));
    assert_eq!(tree.left().unwrap().value(), Some(&2));
    assert_eq!(tree.right().unwrap().value(), Some(&5));

    // Left rotation case (bf < -1, right-heavy)
    //      2
    //       \
    //        5
    //         \
    //          6
    let mut tree2 = BST::new();
    tree2.insert_unbalanced(2);
    tree2.insert_unbalanced(5);
    tree2.insert_unbalanced(6);

    dbg!(&tree2);
    tree2.rebalance();
    dbg!(&tree2);

    assert_eq!(tree2.value(), Some(&5));
    assert_eq!(tree2.left().unwrap().value(), Some(&2));
    assert_eq!(tree2.right().unwrap().value(), Some(&6));

    // Left-Right rotation case
    //      5
    //     /
    //    2
    //     \
    //      3
    let mut tree3 = BST::new();
    tree3.insert_unbalanced(5);
    tree3.insert_unbalanced(2);
    tree3.insert_unbalanced(3);

    dbg!(&tree3);
    tree3.rebalance();
    dbg!(&tree3);

    assert_eq!(tree3.value(), Some(&3));
    assert_eq!(tree3.left().unwrap().value(), Some(&2));
    assert_eq!(tree3.right().unwrap().value(), Some(&5));

    // Right-Left rotation case
    //      2
    //       \
    //        5
    //       /
    //      3
    let mut tree4 = BST::new();
    tree4.insert_unbalanced(2);
    tree4.insert_unbalanced(5);
    tree4.insert_unbalanced(3);

    dbg!(&tree4);
    tree4.rebalance();
    dbg!(&tree4);

    assert_eq!(tree4.value(), Some(&3));
    assert_eq!(tree4.left().unwrap().value(), Some(&2));
    assert_eq!(tree4.right().unwrap().value(), Some(&5));
}
