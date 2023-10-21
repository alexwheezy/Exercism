use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    counter: usize,
}

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            counter: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.counter
    }

    pub fn push(&mut self, _element: T) {
        // Incrementing the counter of the number of nodes
        // in a singly linked list
        self.counter += 1;
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        // Decrementing the counter of the number of nodes
        // in a singly linked list
        if let Some(node) = self.head.take() {
            self.head = node.next;
            self.counter -= 1;
            return Some(node.data);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            return Some(&node.data);
        }
        None
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut linked_list = Self::new();
        let mut tail = self.head;
        while let Some(node) = tail.take() {
            linked_list.push(node.data);
            tail = node.next;
        }
        linked_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut linked_list = SimpleLinkedList::new();
        for _element in _iter {
            linked_list.push(_element)
        }
        linked_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut new_vec = Vec::with_capacity(_linked_list.len());
        let mut reverse_list = _linked_list.rev();
        while let Some(_element) = reverse_list.pop() {
            new_vec.push(_element);
        }
        new_vec
    }
}
