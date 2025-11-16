use core::clone::Clone;
use core::mem::{swap, transmute};

#[cfg(feature = "std")]
use std::sync::Arc;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::sync::Arc;

/// A reverse singly-linked list.
///
/// Each node in the list is reference counted,
/// meaning cloning the list is safe and cheap.
///
/// We're optimizing for cloning the list and
/// appending an item onto its end, both of which
/// are O(1).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List<T> {
    node: Option<Arc<Node<T>>>,
    length: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            node: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn append(&self, value: T) -> Self {
        Self {
            node: Some(Arc::new(Node {
                prev: self.node.clone(),
                value,
            })),
            length: self.length + 1,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node<T> {
    prev: Option<Arc<Node<T>>>,
    value: T,
}

pub struct Iter<'a, T> {
    list: &'a List<T>,
    next: Option<Arc<Node<T>>>,
    node: Option<Arc<Node<T>>>,
}

impl<'a, T> Iter<'a, T> {
    fn new(list: &'a List<T>) -> Self {
        Self {
            list,
            next: None,
            node: list.node.clone(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.node.take();
        swap(&mut self.next, &mut node);
        if let Some(prev) = self.next.as_ref().and_then(|node| node.prev.as_ref()) {
            self.node = Some(Arc::clone(prev));
        }
        self.next.as_ref().map(|next| {
            // SAFETY:
            // We're returning a reference here, but the reference points
            // to the inside of an `Arc<Node<T>>`, meaning the reference
            // to it is valid for as long as the `Arc` lives. It lives for
            // as long as the `list` it came from, which is longer than
            // `self` here.
            // The items within `list` will never be moved around or
            // mutated in any way, because it is immutable. The only
            // supported operation is `append`, which constructs a new
            // list with a pointer to the old one.
            // The borrow checker will ensure that the items do not
            // outlive their parent `list`.
            unsafe { transmute(&next.value) }
        })
    }
}
