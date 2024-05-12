pub struct Node<T: Copy> {
    ch: char,
    continuations: Vec<Node<T>>,
    pub payload: Option<T>
}

impl<'a, T: 'a + Copy> Node<T> {
    pub fn new() -> Self {
        Node {
            ch: 0 as char,
            continuations: Vec::new(),
            payload: None,
        }
    }

    fn from_char(ch: char) -> Self {
        let mut node = Node::new();
        node.ch = ch;
        node
    }

    pub fn insert(&mut self, key: &str, value: T) {
        if key.len() == 0 {
            self.payload = Some(value);
            return
        }

        let ch: char = key.chars().nth(0).unwrap();
        for node in &mut self.continuations {
            if node.ch == ch {
                return node.insert(&key[1..], value)
            }
        }

        let mut node = Node::from_char(ch);
        node.insert(&key[1..], value);
        self.continuations.push(node)
    }

    pub fn cursor(&self) -> Cursor<T> {
        Cursor::new(self)
    }
}

pub struct Cursor<'a, T: 'a + Copy> {
    node: &'a Node<T>,
}

impl<'a, T: Copy> Cursor<'a, T> {
    pub fn new(node: &'a Node<T>) -> Cursor<'a, T> {
        Self { node }
    }

    /// Move forward by one node.
    ///
    /// Returns bool, depicting success of the operation. If returned
    /// false, the passed char isn't a valid continuation. The saved
    /// node stays intact
    pub fn visit(&mut self, ch: char) -> bool {
        for node in &self.node.continuations {
            if node.ch == ch {
                self.node = node;
                return true
            }
        }

        false
    }

    pub fn payload(&self) -> Option<T> {
        match self.node.payload {
            None => None,
            Some(t) => Some(t.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_operator() {
        let mut tree: Node<u32> = Node::new();
        tree.insert("+", 1);
        let mut cursor = tree.cursor();

        assert_eq!(cursor.visit('+'), true);
        assert_eq!(cursor.visit('+'), false);
        assert_eq!(cursor.payload(), Some(1));
    }

    #[test]
    fn composite_operator() {
        let mut tree: Node<u32> = Node::new();
        tree.insert("*", 1);
        tree.insert("**", 2);
        let mut cursor = tree.cursor();

        assert_eq!(cursor.visit('*'), true);
        assert_eq!(cursor.payload(), Some(1));
        assert_eq!(cursor.visit('*'), true);
        assert_eq!(cursor.visit('*'), false);
        assert_eq!(cursor.payload(), Some(2));
    }

    #[test]
    fn unknown_symbol() {
        let mut tree: Node<u32> = Node::new();
        tree.insert("+", 1);
        let mut cursor = tree.cursor();

        assert_eq!(cursor.visit('-'), false);
        assert_eq!(cursor.payload(), None);
    }

    #[test]
    fn separate_operator_from_rest() {
        let mut tree: Node<u32> = Node::new();
        tree.insert("+", 1);
        let mut cursor = tree.cursor();
        assert!(cursor.visit('+'));
        assert!(!cursor.visit('1'));
        assert_eq!(cursor.payload(), Some(1));
    }
}
