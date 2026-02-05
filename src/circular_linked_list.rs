use std::{cell::RefCell, rc::Rc};

/// A circular singly linked list.
///
///
/// [`Cycle`]: std::iter::Cycle
///
/// # Usage
/// ```
/// # use garlic::circular_linked_list::*;
/// // You can push elements to it;
/// let mut cll = CircularLinkedList::new();
/// cll.push("Mike");
/// cll.push("Hank");
/// cll.push("Gus");
///
/// // It also implements FromIterator!
/// let cll: CircularLinkedList<_> = (1..=7).collect();
///
/// ```
///
/// Use [`iter`] to create a never ending iterator that goes from node to node forever.
///
/// Behavior is similar to a the [`Cycle`] iterator, but likely less useful.
/// ```
/// # use garlic::circular_linked_list::*;
/// let cll: CircularLinkedList<_> = "hello".chars().collect();
/// let doubled: String = cll.iter().map_copied().take(cll.len() * 2).collect();
/// assert_eq!(doubled, "hellohello");
/// ```
#[derive(Default)]
pub struct CircularLinkedList<T> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

pub struct Node<T> {
    pub value: T,
    next: Pointer<T>,
}

type Rcrfn<T> = Rc<RefCell<Node<T>>>;
type Pointer<T> = Option<Rcrfn<T>>;

impl<T> CircularLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    /// ## Expensive
    /// Has to traverse entire list.
    pub fn len(&self) -> usize {
        self.iter_once().count()
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, value: T) {
        let Some(rcrfn) = &self.tail else {
            let node = Node { value, next: None };
            let head = Rc::new(RefCell::new(node));
            head.borrow_mut().next = Some(head.clone());

            self.head = Some(head.clone());
            self.tail = Some(head);
            return;
        };

        let next = Node {
            value,
            next: self.head.clone(),
        };
        let next_ptr = Some(Rc::new(RefCell::new(next)));
        rcrfn.borrow_mut().next = next_ptr.clone();
        self.tail = next_ptr.clone();
    }

    /// Creates an iterator that, by default,
    /// will never end, unless the list is empty.
    pub fn iter(&self) -> CllIter<T> {
        CllIter {
            cursor: self.head.clone(),
            tail: self.tail.clone(),
            stop: false,
        }
    }

    /// Creates an iterator that, by default,
    /// will iterate throught the list and stop at the tail element.
    pub fn iter_once(&self) -> CllIter<T> {
        self.iter().once()
    }
}

impl<T> Drop for CircularLinkedList<T> {
    fn drop(&mut self) {
        if let Some(rcrfn) = self.tail.as_mut() {
            rcrfn.borrow_mut().next = None;
        }
    }
}

#[derive(Clone)]
pub struct CllIter<T> {
    cursor: Pointer<T>,
    tail: Pointer<T>,
    stop: bool,
}

impl<T> CllIter<T> {
    pub fn once(mut self) -> Self {
        self.stop = true;
        self
    }

    /// Copies the inner value of each `Rc<RefCell<Node<T>>>`
    ///
    /// Equivelent to `cll.map(|x| x.borrow().value)`
    pub fn map_copied(self) -> impl Iterator<Item = T>
    where
        T: Copy,
    {
        self.map(|x| x.borrow().value)
    }

    /// Clones the inner value of each `Rc<RefCell<Node<T>>>`
    ///
    /// Equivelent to `cll.map(|x| x.borrow().value.clone())`
    pub fn map_cloned(self) -> impl Iterator<Item = T>
    where
        T: Clone,
    {
        self.map(|x| x.borrow().value.clone())
    }
}

impl<T> Iterator for CllIter<T> {
    type Item = Rcrfn<T>;

    fn next(&mut self) -> Pointer<T> {
        let r = self.cursor.take()?;

        if !self.stop || !Rc::ptr_eq(&r, self.tail.as_ref().unwrap()) {
            self.cursor = r.borrow().next.clone();
        }

        Some(r)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for CircularLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut l = f.debug_list();
        for rcrfn in self.iter_once() {
            l.entry(&rcrfn.borrow().value);
        }
        l.finish()
    }
}

impl<T> std::iter::FromIterator<T> for CircularLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(Self::new(), |mut cll, x| {
            cll.push(x);
            cll
        })
    }
}
