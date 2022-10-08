enum Direction {
    CLOCKWISE,
    COUNTERCLOCKWISE,
}

enum Child {
    RIGHT,
    LEFT,
}

struct Node<T: Ord> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(data: T) -> Self {
        Self { data: data, left: None, right: None }
    }

    fn rotate_right(mut self) -> Box<Node<T>> {
        if let Some(mut node) = self.left.take() {
            self.left = node.right.take();
            node.right = Some(Box::new(self));
            return node;
        }

        Box::new(self)
    }

    fn rotate_left(mut self) -> Box<Node<T>> {
        if let Some(mut node) = self.right.take() {
            self.right = node.left.take();
            node.left = Some(Box::new(self));
            return node;
        }

        Box::new(self)
    }

    fn rotate(&mut self, child: Child, direction: Direction) {
        let node_ref = match child {
            Child::RIGHT => &mut self.right,
            Child::LEFT => &mut self.left,
        };

        if let Some(node) = node_ref.take() {
            let _ = node_ref.insert( match direction {
                Direction::CLOCKWISE => node.rotate_right(),
                Direction::COUNTERCLOCKWISE => node.rotate_left(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_right_should_not_fail() {
        let mut left_tree = Node::<char>::new('p');
        left_tree.left = Some(Box::new(Node::<char>::new('a')));
        left_tree.right = Some(Box::new(Node::<char>::new('b')));

        let mut root = Node::<char>::new('q');
        root.left = Some(Box::new(left_tree));
        root.right = Some(Box::new(Node::<char>::new('c')));

        let rotated_root = root.rotate_right();
        
        assert_eq!(rotated_root.data, 'p');
        
        let rotated_left_tree = rotated_root.left.unwrap();
        assert_eq!(rotated_left_tree.data, 'a');
        assert!(rotated_left_tree.left.is_none());
        assert!(rotated_left_tree.right.is_none());
        
        let rotated_right_tree = rotated_root.right.unwrap();
        assert_eq!(rotated_right_tree.data, 'q');

        let rotated_right_left_tree = rotated_right_tree.left.unwrap();
        assert_eq!(rotated_right_left_tree.data, 'b');
        assert!(rotated_right_left_tree.left.is_none());
        assert!(rotated_right_left_tree.right.is_none());

        let rotated_right_right_tree = rotated_right_tree.right.unwrap();
        assert_eq!(rotated_right_right_tree.data, 'c');
        assert!(rotated_right_right_tree.left.is_none());
        assert!(rotated_right_right_tree.right.is_none());
    }

    #[test]
    fn rotate_left_should_not_fail() {
        let mut right_tree = Node::<char>::new('q');
        right_tree.left = Some(Box::new(Node::<char>::new('b')));
        right_tree.right = Some(Box::new(Node::<char>::new('c')));

        let mut root = Node::<char>::new('p');
        root.left = Some(Box::new(Node::<char>::new('a')));
        root.right = Some(Box::new(right_tree));

        let rotated_root = root.rotate_left();
        
        assert_eq!(rotated_root.data, 'q');
        
        let rotated_right_tree = rotated_root.right.unwrap();
        assert_eq!(rotated_right_tree.data, 'c');
        assert!(rotated_right_tree.left.is_none());
        assert!(rotated_right_tree.right.is_none());
        
        let rotated_left_tree = rotated_root.left.unwrap();
        assert_eq!(rotated_left_tree.data, 'p');

        let rotated_left_left_tree = rotated_left_tree.left.unwrap();
        assert_eq!(rotated_left_left_tree.data, 'a');
        assert!(rotated_left_left_tree.left.is_none());
        assert!(rotated_left_left_tree.right.is_none());

        let rotated_left_right_tree = rotated_left_tree.right.unwrap();
        assert_eq!(rotated_left_right_tree.data, 'b');
        assert!(rotated_left_right_tree.left.is_none());
        assert!(rotated_left_right_tree.right.is_none());
    }
}