#[cfg(test)]
mod btree_tests {
    use dsa::btree::*;

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
    fn test_postorder() {
        let mut tree = BinaryTree::new();
        tree.insert(2);
        tree.insert(5);
        tree.insert(4);
        tree.insert(1);
        tree.insert(3);
        assert_eq!(vec![5, 4, 3, 2, 1], tree.get_postorder());
    }
}
