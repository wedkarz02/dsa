pub struct Node<T: Ord + Copy> {
    pub value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn min(&self) -> Option<&Node<T>> {
        if let Some(node) = &self.left {
            node.min()
        } else {
            Some(self)
        }
    }
}

pub struct BinaryTree<T: Ord + Copy> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        match &mut self.root {
            Some(node) => BinaryTree::insert_rec(value, node),
            None => self.root = Some(Box::new(Node::new(value))),
        }
    }

    fn insert_rec(value: T, current: &mut Box<Node<T>>) {
        if value < current.value {
            match &mut current.left {
                Some(node) => BinaryTree::insert_rec(value, node),
                None => current.left = Some(Box::new(Node::new(value))),
            }
        } else {
            match &mut current.right {
                Some(node) => BinaryTree::insert_rec(value, node),
                None => current.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    pub fn min(&self) -> Option<&Node<T>> {
        self.root.as_ref().and_then(|node| node.min())
    }

    pub fn get_inorder(&self) -> Vec<T> {
        let mut values: Vec<T> = Vec::new();

        if let Some(node) = &self.root {
            BinaryTree::get_inorder_rec(node, &mut values);
        }

        values
    }

    fn get_inorder_rec(current: &Box<Node<T>>, values: &mut Vec<T>) {
        if let Some(node) = &current.left {
            BinaryTree::get_inorder_rec(node, values);
        }

        values.push(current.value);

        if let Some(node) = &current.right {
            BinaryTree::get_inorder_rec(node, values);
        }
    }

    pub fn get_postorder(&self) -> Vec<T> {
        let mut values: Vec<T> = Vec::new();

        if let Some(node) = &self.root {
            BinaryTree::get_postorder_rec(node, &mut values);
        }

        values
    }

    fn get_postorder_rec(current: &Box<Node<T>>, values: &mut Vec<T>) {
        if let Some(node) = &current.right {
            BinaryTree::get_postorder_rec(node, values);
        }

        values.push(current.value);

        if let Some(node) = &current.left {
            BinaryTree::get_postorder_rec(node, values);
        }
    }
}
