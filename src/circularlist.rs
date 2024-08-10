use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Node<T: Clone> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Clone, Debug)]
pub struct CircularList<T: Clone> {
    pub head: Option<Rc<RefCell<Node<T>>>>,
    pub size: usize,
}

impl<T: Clone + Debug> CircularList<T> {
    pub fn new() -> Self {
        CircularList {
            head: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, value: T, index: usize) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }));

        if self.head.is_none() {
            new_node.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::clone(&new_node));
            self.head = Some(new_node);
        } else {
            let mut current = Rc::clone(self.head.as_ref().unwrap());
            for _ in 0..index {
                let next = Rc::clone(current.borrow().next.as_ref().unwrap());
                current = next;
            }

            let next = Rc::clone(current.borrow().next.as_ref().unwrap());

            new_node.borrow_mut().next = Some(Rc::clone(&next));
            new_node.borrow_mut().prev = Some(Rc::clone(&current));

            current.borrow_mut().next = Some(Rc::clone(&new_node));
            next.borrow_mut().prev = Some(Rc::clone(&new_node));

            if index == 0 {
                self.head = Some(Rc::clone(&new_node));
            }
        }

        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size || self.head.is_none() {
            return None;
        }

        let mut current = Rc::clone(self.head.as_ref().unwrap());
        for _ in 0..index {
            let next = Rc::clone(current.borrow().next.as_ref().unwrap());
            current = next;
        }

        let prev = Rc::clone(current.borrow().prev.as_ref().unwrap());
        let next = Rc::clone(current.borrow().next.as_ref().unwrap());

        prev.borrow_mut().next = Some(Rc::clone(&next));
        next.borrow_mut().prev = Some(Rc::clone(&prev));

        if index == 0 {
            self.head = if self.size == 1 {
                None
            } else {
                Some(Rc::clone(&next))
            };
        }

        self.size -= 1;
        let value = current.borrow().value.clone();
        Some(value)
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.size || self.head.is_none() {
            return None;
        }

        let mut current = Rc::clone(self.head.as_ref().unwrap());
        for _ in 0..index {
            let next = Rc::clone(current.borrow().next.as_ref().unwrap());
            current = next;
        }

        let value = current.borrow().value.clone();
        Some(value)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }

    pub fn iter(&self) -> CircularListIterator<T> {
        CircularListIterator {
            current: self.head.clone(),
            end: self.head.clone(),
            first: true,
        }
    }
}

pub struct CircularListIterator<T: Clone> {
    current: Option<Rc<RefCell<Node<T>>>>,
    end: Option<Rc<RefCell<Node<T>>>>,
    first: bool,
}

impl<T: Clone + Debug> Iterator for CircularListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            return None;
        }

        if !self.first && Rc::ptr_eq(self.current.as_ref().unwrap(), self.end.as_ref().unwrap()) {
            return None;
        }

        let value = self.current.as_ref().unwrap().borrow().value.clone();
        let next = self.current.as_ref().unwrap().borrow().next.clone();

        self.current = next;
        self.first = false;

        Some(value)
    }
}
