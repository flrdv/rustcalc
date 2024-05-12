pub struct Node<T: 'static> {
    ch: char,
    continuations: Vec<Node<T>>,
    pub payload: Option<&'static T>
}

impl<T: 'static> Node<T> {
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

    pub fn insert(&mut self, key: &str, value: &'static T) {
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

        let node = Node::from_char(ch);
        self.continuations.push(node)
    }

    pub fn cursor(&self) -> Cursor<T> {
        Cursor::new(self)
    }
}

struct Cursor<'a, T: 'static> {
    node: &'a Node<T>,
}

impl<'a, T> Cursor<'a, T> {
    pub fn new(node: &'a Node<T>) -> Cursor<'a, T> {
        Self { node }
    }

    /// Go forwards by one node
    ///
    /// Returns bool, depicting success of the operation. If returned
    /// false, the passed char isn't a valid continuation
    pub fn visit(&mut self, ch: char) -> bool {
        for node in &self.node.continuations {
            if node.ch == ch {
                self.node = node;
                return true
            }
        }

        false
    }

    pub fn payload(&self) -> Option<&'static T> {
        self.node.payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid() {
        let mut tree: Node<u32> = Node::new();
        tree.insert("+", &1);
        tree.insert("-", &2);
        tree.insert("*", &3);
        tree.insert("**", &4);


    }
}
