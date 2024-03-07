use std::cmp;

pub struct BinaryTree<T: Ord + Copy> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Copy> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => node.insert(value),
            None => self.root = Some(Box::new(Node::new(value))),
        }
    }

    pub fn delete(&mut self, value: &T) {
        Node::delete(&mut self.root, value);
    }

    pub fn get_inorder(&self) -> Vec<T> {
        let mut values: Vec<T> = Vec::new();
        if let Some(ref node) = self.root {
            node.push_inorder(&mut values);
        }
        values
    }
}

pub struct Node<T: Ord + Copy> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            cmp::Ordering::Less => match self.left {
                Some(ref mut node) => node.insert(value),
                None => {
                    self.left = Some(Box::new(Node::new(value)));
                }
            },
            cmp::Ordering::Greater => match self.right {
                Some(ref mut node) => node.insert(value),
                None => {
                    self.right = Some(Box::new(Node::new(value)));
                }
            },
            cmp::Ordering::Equal => {}
        }
    }

    fn delete(root: &mut Option<Box<Node<T>>>, value: &T) {
        if let Some(ref mut node) = root {
            match value.cmp(&node.value) {
                cmp::Ordering::Less => Node::delete(&mut node.left, value),
                cmp::Ordering::Greater => Node::delete(&mut node.right, value),
                cmp::Ordering::Equal => match (&node.left, &node.right) {
                    (None, None) => *root = None,
                    (Some(_), None) => *root = node.left.take(),
                    (None, Some(_)) => *root = node.right.take(),
                    (Some(_), Some(_)) => node.value = Node::delete_min(&mut node.right).unwrap(),
                },
            }
        }
    }

    fn delete_min(root: &mut Option<Box<Node<T>>>) -> Option<T> {
        if root.as_ref().unwrap().left.is_some() {
            Node::delete_min(&mut root.as_mut().unwrap().left)
        } else {
            let node = root.take().unwrap();
            *root = node.right;
            Some(node.value)
        }
    }

    fn push_inorder(&self, vals: &mut Vec<T>) {
        if let Some(ref node) = self.left {
            node.push_inorder(vals);
        }

        vals.push(self.value);

        if let Some(ref node) = self.right {
            node.push_inorder(vals);
        }
    }
}
