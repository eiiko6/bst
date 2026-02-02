extern crate std;

use crate::BST;

#[test]
fn test_count_and_depth() {
    let mut tree = BST::new();
    tree.insert(5);
    tree.insert(8);
    tree.insert(3);
    tree.insert(2);
    tree.insert(6);
    tree.insert(9);
    tree.insert(10);
    tree.insert(9);

    dbg!(&tree);

    assert_eq!(tree.count_nodes(), 7);
    assert_eq!(tree.depth(), 4);
}

#[test]
fn test_find() {
    let mut tree = BST::new();
    tree.insert(5);
    tree.insert(8);
    tree.insert(3);
    tree.insert(2);
    tree.insert(9);
    tree.insert(7);

    dbg!(&tree);

    assert!(tree.contains(&2));
    assert_eq!(tree.find(&2).unwrap().root_value().unwrap(), &2);
    assert_eq!(tree.find(&7).unwrap().root_value().unwrap(), &7);
    assert_eq!(tree.find(&5).unwrap().root_value().unwrap(), &5);
    assert!(tree.find(&999).is_none());
}

#[test]
fn test_ends() {
    let mut tree = BST::new();
    tree.insert(5);
    tree.insert(8);
    tree.insert(3);
    tree.insert(2);
    tree.insert(9);
    tree.insert(7);

    dbg!(&tree);

    assert_eq!(tree.left_end().unwrap().root_value().unwrap(), &2);
    assert_eq!(tree.right_end().unwrap().root_value().unwrap(), &9);
}

#[test]
fn test_equals() {
    let mut tree1 = BST::new();
    tree1.insert(5);
    tree1.insert(8);
    tree1.insert(3);
    tree1.insert(2);
    tree1.insert(9);
    tree1.insert(7);

    let mut tree2 = BST::new();
    tree2.insert(5);
    tree2.insert(8);
    tree2.insert(3);
    tree2.insert(2);
    tree2.insert(9);
    tree2.insert(7);

    assert!(tree1 == tree2);
    tree2.insert(10);
    assert!(tree1 != tree2);
}

#[test]
fn test_clone_equals() {
    let mut tree1 = BST::new();
    tree1.insert(5);
    tree1.insert(8);
    tree1.insert(3);
    tree1.insert(2);
    tree1.insert(9);
    tree1.insert(7);

    let tree2 = tree1.clone();

    assert!(tree1 == tree2);
}

#[test]
fn test_balance_factor() {
    let mut tree = BST::new();
    tree.insert(6);
    tree.insert(1);
    tree.insert(2);
    tree.insert(8);
    tree.insert(9);
    tree.insert(10);
    tree.insert(11);
    tree.insert(12);

    dbg!(&tree);

    let bf = tree.balance_factor();
    assert!(bf.abs() <= 1, "Tree should be balanced, but bf is {}", bf);

    tree.insert(3);
    tree.insert(4);
    tree.insert(5);

    dbg!(&tree);

    let bf_after = tree.balance_factor();
    assert!(
        bf_after.abs() <= 1,
        "Tree should remain balanced, but bf is {}",
        bf_after
    );
}

#[test]
fn test_insert_maintains_balance() {
    //      5
    //     /
    //    3
    //   /
    //  2
    let mut tree = BST::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(2);

    dbg!(&tree);

    // Should balance to:
    //    3
    //   / \
    //  2   5
    assert_eq!(tree.root_value(), Some(&3));
    assert_eq!(tree.left().unwrap().root_value(), Some(&2));
    assert_eq!(tree.right().unwrap().root_value(), Some(&5));

    //      2
    //       \
    //        5
    //         \
    //          6
    let mut tree2 = BST::new();
    tree2.insert(2);
    tree2.insert(5);
    tree2.insert(6);

    dbg!(&tree2);

    // Should balance to:
    //    5
    //   / \
    //  2   6
    assert_eq!(tree2.root_value(), Some(&5));
    assert_eq!(tree2.left().unwrap().root_value(), Some(&2));
    assert_eq!(tree2.right().unwrap().root_value(), Some(&6));

    //      5
    //     /
    //    2
    //     \
    //      3
    let mut tree3 = BST::new();
    tree3.insert(5);
    tree3.insert(2);
    tree3.insert(3);

    dbg!(&tree3);

    // Should balance to:
    //    3
    //   / \
    //  2   5
    assert_eq!(tree3.root_value(), Some(&3));
    assert_eq!(tree3.left().unwrap().root_value(), Some(&2));
    assert_eq!(tree3.right().unwrap().root_value(), Some(&5));

    //      2
    //       \
    //        5
    //       /
    //      3
    let mut tree4 = BST::new();
    tree4.insert(2);
    tree4.insert(5);
    tree4.insert(3);

    dbg!(&tree4);

    // Should balance to:
    //    3
    //   / \
    //  2   5
    assert_eq!(tree4.root_value(), Some(&3));
    assert_eq!(tree4.left().unwrap().root_value(), Some(&2));
    assert_eq!(tree4.right().unwrap().root_value(), Some(&5));
}

#[test]
fn test_remove() {
    let mut tree = BST::new();

    for val in [10, 5, 15, 2, 7, 12, 18, 1] {
        tree.insert(val);
        dbg!(&tree);
    }

    let mut initial_count = tree.count_nodes();

    println!("removing 18");
    tree.remove(&18);
    dbg!(&tree);
    let count = tree.count_nodes();
    initial_count -= 1;
    assert_eq!(
        count, initial_count,
        "Tree is of size {} but should be {}",
        count, initial_count
    );
    let bf = tree.balance_factor();
    assert!(bf.abs() <= 1, "Tree is unbalanced: bf = {}", bf);

    println!("removing 12");
    tree.remove(&12);
    dbg!(&tree);
    let count = tree.count_nodes();
    initial_count -= 1;
    assert_eq!(
        count, initial_count,
        "Tree is of size {} but should be {}",
        count, initial_count
    );
    let bf = tree.balance_factor();
    assert!(bf.abs() <= 1, "Tree is unbalanced: bf = {}", bf);

    println!("removing 15");
    tree.remove(&15);
    dbg!(&tree);
    let count = tree.count_nodes();
    initial_count -= 1;
    assert_eq!(
        count, initial_count,
        "Tree is of size {} but should be {}",
        count, initial_count
    );
    let bf = tree.balance_factor();
    assert!(bf.abs() <= 1, "Tree is unbalanced: bf = {}", bf);
}

#[test]
fn test_depth() {
    let mut tree = BST::new();
    tree.insert(10);
    assert_eq!(tree.depth(), 1);

    tree.insert(20);
    assert_eq!(tree.depth(), 2);

    tree.insert(30);
    // Structure should be:
    //    20
    //  10  30
    assert_eq!(tree.root_value(), Some(&20));
    assert_eq!(tree.depth(), 2);

    tree.insert(40);
    tree.insert(50);
    assert!(tree.depth() >= 3);

    assert!(tree.balance_factor().abs() <= 1);
}
