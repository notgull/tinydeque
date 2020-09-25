// MIT/Apache2 License

#![cfg(feature = "alloc")]

use super::array_deque::{ArrayDeque, Iter as ArrayDequeIter};
use alloc::collections::vec_deque::{Iter as VecDequeIter, VecDeque};
use core::iter::FromIterator;
use tinyvec::Array;

/// A deque structure that can overflow onto the heap if it spills the stack.
pub enum TinyDeque<A: Array> {
    Stack(ArrayDeque<A>),
    Heap(VecDeque<A::Item>),
}

impl<A: Array> TinyDeque<A> {
    /// Create a new `TinyDeque`.
    #[inline]
    pub fn new() -> Self {
        Self::Stack(ArrayDeque::new())
    }

    /// Create a new `TinyDeque` with the specified capacity. If the capacity is greater
    /// than the array capacity, it will spill onto the heap.
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        if cap > A::CAPACITY {
            Self::Stack(ArrayDeque::new())
        } else {
            Self::Heap(VecDeque::with_capacity(cap))
        }
    }

    /// Get the length of this `TinyDeque`.
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Stack(a) => a.len(),
            Self::Heap(v) => v.len(),
        }
    }

    /// Tell if this `TinyDeque` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Stack(a) => a.is_empty(),
            Self::Heap(v) => v.is_empty(),
        }
    }

    /// Push an element onto the back of this deque.
    #[inline]
    pub fn push_back(&mut self, element: A::Item) {
        match self {
            Self::Heap(v) => v.push_back(element),
            Self::Stack(s) => {
                if let Err(reject) = s.try_push_back(element) {
                    self.spill();
                    self.as_heap_mut().push_back(reject);
                }
            }
        }
    }

    /// Push an element onto the front of this deque.
    #[inline]
    pub fn push_front(&mut self, element: A::Item) {
        match self {
            Self::Heap(v) => v.push_back(element),
            Self::Stack(s) => {
                if let Err(reject) = s.try_push_back(element) {
                    self.spill();
                    self.as_heap_mut().push_back(reject);
                }
            }
        }
    }

    /// Pop an element from the back of this deque.
    #[inline]
    pub fn pop_back(&mut self) -> Option<A::Item> {
        match self {
            Self::Heap(v) => v.pop_back(),
            Self::Stack(s) => s.pop_back(),
        }
    }

    /// Pop an element from the front of this deque.
    #[inline]
    pub fn pop_front(&mut self) -> Option<A::Item> {
        match self {
            Self::Heap(v) => v.pop_front(),
            Self::Stack(s) => s.pop_front(),
        }
    }

    /// Get a reference to an element in the deque.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&A::Item> {
        match self {
            Self::Heap(v) => v.get(index),
            Self::Stack(s) => s.get(index),
        }
    }

    /// Get a mutable reference to an element in the deque.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut A::Item> {
        match self {
            Self::Heap(v) => v.get_mut(index),
            Self::Stack(s) => s.get_mut(index),
        }
    }

    /// Truncate this `TinyDeque` to be a certain length.
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        match self {
            Self::Heap(v) => v.truncate(len),
            Self::Stack(s) => s.truncate(len),
        }
    }

    /// Remove all elements from this `TinyDeque`.
    #[inline]
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Get two slices that contain this `TinyDeque`'s contents.
    #[inline]
    pub fn as_slices(&self) -> (&[A::Item], &[A::Item]) {
        match self {
            Self::Heap(v) => v.as_slices(),
            Self::Stack(s) => s.as_slices(),
        }
    }

    /// Get two mutable slices that contain this `TinyDeque`'s contents.
    #[inline]
    pub fn as_mut_slices(&mut self) -> (&mut [A::Item], &mut [A::Item]) {
        match self {
            Self::Heap(v) => v.as_mut_slices(),
            Self::Stack(s) => s.as_mut_slices(),
        }
    }

    /// Create an iterator.
    #[inline]
    pub fn iter(&self) -> Iter<'_, A> {
        match self {
            Self::Heap(v) => Iter::Heap(v.iter()),
            Self::Stack(s) => Iter::Stack(s.iter()),
        }
    }

    #[inline]
    fn as_heap_mut(&mut self) -> &mut VecDeque<A::Item> {
        match self {
            Self::Heap(h) => h,
            Self::Stack(_) => unreachable!(),
        }
    }

    #[inline]
    fn spill(&mut self) {
        let stack = match self {
            Self::Heap(_) => return,
            Self::Stack(ref mut s) => s,
        };
        let mut heap = VecDeque::with_capacity(stack.len() + 1);
        while let Some(item) = stack.pop_front() {
            heap.push_back(item);
        }
        *self = Self::Heap(heap);
    }
}

impl<A: Array> Clone for TinyDeque<A>
where
    A::Item: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::Stack(a) => Self::Stack(a.clone()),
            Self::Heap(v) => Self::Heap(v.clone()),
        }
    }
}

impl<A: Array> FromIterator<A::Item> for TinyDeque<A> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = A::Item>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut me = Self::with_capacity(iter.size_hint().0);
        me.extend(iter);
        me
    }
}

impl<A: Array> Extend<A::Item> for TinyDeque<A> {
    #[inline]
    fn extend<T: IntoIterator<Item = A::Item>>(&mut self, iter: T) {
        iter.into_iter().for_each(|item| self.push_back(item));
    }
}

/// An iterator over the elements in a `TinyDeque`.
pub enum Iter<'a, A: Array> {
    Stack(ArrayDequeIter<'a, A>),
    Heap(VecDequeIter<'a, A::Item>),
}

impl<'a, A: Array> Iterator for Iter<'a, A> {
    type Item = &'a A::Item;

    #[inline]
    fn next(&mut self) -> Option<&'a A::Item> {
        match self {
            Self::Stack(a) => a.next(),
            Self::Heap(v) => v.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Stack(a) => a.size_hint(),
            Self::Heap(v) => v.size_hint(),
        }
    }
}

impl<'a, A: Array> ExactSizeIterator for Iter<'a, A> {}

impl<'a, A: Array> DoubleEndedIterator for Iter<'a, A> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a A::Item> {
        match self {
            Self::Stack(a) => a.next_back(),
            Self::Heap(v) => v.next_back(),
        }
    }
}
