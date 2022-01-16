use std::{cell::RefCell, rc::Rc};

pub trait StringListIterator
where
    Self: Iterator,
{
    fn reset(&mut self);
}

pub struct StringListDefaultIterator {
    index: usize,
    items: Rc<RefCell<Vec<String>>>,
}

impl StringListDefaultIterator {
    pub fn new(items: Rc<RefCell<Vec<String>>>) -> Self {
        Self { index: 0, items }
    }
}

impl Iterator for StringListDefaultIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let borrow = self.items.borrow();
        let next = borrow.get(self.index);

        match next {
            Some(next) => {
                self.index += 1;
                Some(String::from(next))
            }
            None => None,
        }
    }
}

impl StringListIterator for StringListDefaultIterator {
    fn reset(&mut self) {
        self.index = 0;
    }
}

pub struct StringListReverseIterator {
    index: usize,
    items: Rc<RefCell<Vec<String>>>,
    done: bool,
}

impl StringListReverseIterator {
    pub fn new(items: Rc<RefCell<Vec<String>>>) -> Self {
        let index = items.borrow().len();
        Self {
            index,
            items,
            done: false,
        }
    }
}

impl Iterator for StringListReverseIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if self.index == 1 && !self.done {
            self.done = true;
        }

        let borrow = self.items.borrow();
        let index = if self.done { 0 } else { self.index - 1 };
        let next = borrow.get(index);

        match next {
            Some(next) => {
                if !self.done {
                    self.index -= 1;
                }

                Some(String::from(next))
            }
            None => None,
        }
    }
}

impl StringListIterator for StringListReverseIterator {
    fn reset(&mut self) {
        self.index = self.items.borrow().len();
        self.done = false;
    }
}

pub struct StringList {
    items: Rc<RefCell<Vec<String>>>,
    iter: AnyStringListIterator,
}

type AnyStringListIterator = Box<dyn StringListIterator<Item = String>>;

impl StringList {
    pub fn new() -> Self {
        let items = Rc::new(RefCell::new(vec![]));
        let iter = StringListDefaultIterator::new(Rc::clone(&items));
        let iter = Box::new(iter);

        Self { items, iter }
    }

    pub fn add_items(&self, items: Vec<&str>) {
        let mut items = items.iter().map(|&s| String::from(s)).collect();
        self.items.borrow_mut().append(&mut items);
    }

    pub fn reset_iterator(&mut self) {
        self.iter.reset();
    }

    pub fn size(&self) -> usize {
        self.items.borrow().len()
    }

    /// Get a reference to the string list's items.
    pub fn items(&self) -> Rc<RefCell<Vec<String>>> {
        Rc::clone(&self.items)
    }

    /// Get a mutable reference to the string list's iterator.
    pub fn iter(&mut self) -> &mut AnyStringListIterator {
        &mut self.iter
    }

    /// Set the string list's iterator.
    pub fn set_iterator(&mut self, iterator: AnyStringListIterator) {
        self.iter = iterator;
    }
}
