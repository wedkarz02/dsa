#[cfg(test)]
mod bst_tests {
    use dsa::bst::*;

    #[test]
    fn test_inorder() {
        let mut tree = BinaryTree::new();
        tree.insert(2);
        tree.insert(5);
        tree.insert(4);
        tree.insert(1);
        tree.insert(3);
        assert_eq!(vec![1, 2, 3, 4, 5], tree.get_inorder());
    }

    #[test]
    fn test_delete() {
        let mut tree = BinaryTree::new();
        tree.insert(4);
        tree.insert(2);
        tree.insert(5);
        tree.insert(1);
        tree.insert(3);

        tree.delete(&4);
        assert_eq!(vec![1, 2, 3, 5], tree.get_inorder());
        tree.delete(&1);
        assert_eq!(vec![2, 3, 5], tree.get_inorder());
        tree.delete(&3);
        assert_eq!(vec![2, 5], tree.get_inorder());
        tree.delete(&5);
        assert_eq!(vec![2], tree.get_inorder());
        tree.delete(&2);
        assert!(tree.get_inorder().is_empty());
    }
}
