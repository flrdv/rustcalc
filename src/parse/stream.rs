pub struct Stream<T> {
    items: Vec<T>,
    position: usize,
}

impl<T> Stream<T> {
    pub fn new(from: Vec<T>) -> Self {
        Stream {
            items: from,
            position: 0,
        }
    }

    pub fn pop(&mut self) -> Option<&T> {
        if self.position >= self.items.len() {
            return None
        }

        self.position += 1;
        Some(&self.items[self.position-1])
    }

    pub fn back(&mut self) {
        if self.position > 0 {
            self.position -= 1
        }
    }

    pub fn len(&self) -> usize {
        self.items.len() - self.position
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> core::slice::Iter<T> {
        self.items.iter()
    }
}

mod tests {
    use crate::parse::stream::Stream;

    #[test]
    fn pop_from_empty() {
        let mut stream = Stream::new(vec![1, 2, 3]);
        assert!(!stream.empty());
        assert_eq!(stream.len(), 3);
        assert_eq!(stream.pop(), Some(&1));
        assert_eq!(stream.pop(), Some(&2));
        assert_eq!(stream.pop(), Some(&3));
        assert_eq!(stream.pop(), None);
        assert!(stream.empty());
        assert_eq!(stream.len(), 0);
    }

    #[test]
    fn back() {
        let mut stream = Stream::new(vec![1, 2]);
        assert_eq!(stream.pop(), Some(&1));
        assert_eq!(stream.pop(), Some(&2));
        assert_eq!(stream.pop(), None);
        stream.back();
        assert_eq!(stream.pop(), Some(&2));
    }
}
