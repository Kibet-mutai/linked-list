use std::mem;

struct Node<T> {
    data: T,
    next: Link<T>,
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: u32,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            next: Link::Empty,
        }
    }

    fn append(&mut self, data: Box<Node<T>>) {
        self.next = Link::More(data);
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: Link::Empty,
            tail: Link::Empty,
            length: 0,
        }
    }

    pub fn push(&mut self, data: T) {
        let node: Box<Node<T>> = Box::new(Node::new(data));
        match mem::replace(&mut self.tail, Link::Empty) {
            Link::Empty => self.head = Link::More(node),
            Link::More(ref mut tail) => tail.append(node),
        }
        self.length += 1
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;

                self.length -= 1;
                Some(node.data)
            }
        }
    }

    pub fn exists(&mut self) -> bool {
        match self.head {
            Link::Empty => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_test() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.exists(), false);
        list.push(1);
        assert_eq!(list.exists(), true);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.length, 0);
    }
}
