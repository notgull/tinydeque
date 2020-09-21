// MIT/Apache2 License

#![allow(clippy::redundant_pattern_matching)]
#![warn(clippy::pedantic)]

use core::{
    iter::{FromIterator, FusedIterator},
    mem,
};
use tinyvec::Array;

/// A deque structure that uses an array as backing storage.
///
/// # Example
///
/// ```
/// use tinydeque::ArrayDeque;
///
/// // lots of people try to cut through the line at the DMV, and that's a dick move
/// // if I've ever seen one. Let's make a program to keep track
/// let mut dmv_line: ArrayDeque<[&'static str; 3]> = ArrayDeque::new();
/// dmv_line.push_back("John");
/// dmv_line.push_back("Thomas");
/// dmv_line.push_back("Larson");
///
/// // make sure the line isn't empty
/// assert!(!dmv_line.is_empty());
/// assert_eq!(dmv_line.len(), 3);
///
/// // if we push another item into the line, it will fail
/// assert!(dmv_line.try_push_back("Carson").is_err());
///
/// // NEXT!
/// assert_eq!(dmv_line.pop_front(), Some("John"));
///
/// // we have a VIP, front of the line!
/// dmv_line.push_front("J.J. Abrams");
/// assert_eq!(dmv_line.pop_front(), Some("J.J. Abrams"));
///
/// // why did J.J. Abrams get to cut in front of me in line?
/// // fuck this, I'm out of here
/// assert_eq!(dmv_line.pop_back(), Some("Larson"));
/// ```
pub struct ArrayDeque<A: Array> {
    ring_buffer: A,
    head: usize,
    tail: usize,
}

impl<A: Array> Default for ArrayDeque<A> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Array> ArrayDeque<A> {
    /// Create a new `ArrayDeque`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            ring_buffer: A::default(),
            head: 0,
            tail: 0,
        }
    }

    /// The capacity of this `ArrayDeque`.
    #[inline]
    #[must_use]
    pub fn capacity() -> usize {
        A::CAPACITY
    }

    /// Helper function to get the wrapped index.
    #[inline]
    fn wrap_index(index: usize, size: usize) -> usize {
        index % size
    }

    /// Helper function to do a wrapping add of an index.
    #[inline]
    fn wrap_add(&self, index: usize, add: usize) -> usize {
        Self::wrap_index(index.wrapping_add(add), Self::capacity())
    }

    /// Helper function to do a wrapping sub of an index.
    #[inline]
    fn wrap_sub(&self, index: usize, sub: usize) -> usize {
        Self::wrap_index(index.wrapping_sub(sub), Self::capacity())
    }

    /// Helper function to get len.
    #[inline]
    fn count(tail: usize, head: usize, size: usize) -> usize {
        (head.wrapping_sub(tail)) % size
    }

    /// Get the length of this `ArrayDeque`.
    #[inline]
    pub fn len(&self) -> usize {
        Self::count(self.tail, self.head, Self::capacity())
    }

    /// Tell whether this `ArrayDeque` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tail == self.head
    }

    /// Tell whether this `ArrayDeque` is full.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == Self::capacity()
    }

    /// Push an element onto the back of this `ArrayDeque`.
    ///
    /// # Errors
    ///
    /// If this `ArrayDeque` is full, this function returns an Err with the rejected element.
    #[inline]
    pub fn try_push_back(&mut self, element: A::Item) -> Result<(), A::Item> {
        // if we're full, error out
        if self.is_full() {
            return Err(element);
        }

        // increment the head by one
        let head = self.head;
        self.head = self.wrap_add(self.head, 1);
        self.ring_buffer.as_slice_mut()[head] = element;
        Ok(())
    }

    /// Push an element onto the back of this `ArrayDeque`.
    ///
    /// # Panics
    ///
    /// This function will panic if the `ArrayDeque` is full.
    #[inline]
    pub fn push_back(&mut self, element: A::Item) {
        if let Err(_) = self.try_push_back(element) {
            panic!("<ArrayDeque> Unable to push element onto ArrayDeque, since it is full");
        }
    }

    /// Push an element onto the front of this `ArrayDeque`.
    ///
    /// # Errors
    ///
    /// If this `ArrayDeque` is full, this function returns an Err with the rejected element.
    #[inline]
    pub fn try_push_front(&mut self, element: A::Item) -> Result<(), A::Item> {
        // if we're full, error out
        if self.is_full() {
            return Err(element);
        }

        self.tail = self.wrap_sub(self.tail, 1);
        self.ring_buffer.as_slice_mut()[self.tail] = element;
        Ok(())
    }

    /// Push an element onto the front of this `ArrayDeque`.
    ///
    /// # Panics
    ///
    /// This function will panic if the `ArrayDeque` is full.
    #[inline]
    pub fn push_front(&mut self, element: A::Item) {
        if let Err(_) = self.try_push_front(element) {
            panic!("<ArrayDeque> Unable to push element onto ArrayDeque, since it is full");
        }
    }

    /// Pop an element from the back of this `ArrayDeque`.
    #[inline]
    pub fn pop_back(&mut self) -> Option<A::Item> {
        if self.is_empty() {
            None
        } else {
            self.head = self.wrap_sub(self.head, 1);
            Some(mem::take(&mut self.ring_buffer.as_slice_mut()[self.head]))
        }
    }

    /// Pop an element from the front of this `ArrayDeque`.
    #[inline]
    pub fn pop_front(&mut self) -> Option<A::Item> {
        if self.is_empty() {
            None
        } else {
            let tail = self.tail;
            self.tail = self.wrap_add(self.tail, 1);
            Some(mem::take(&mut self.ring_buffer.as_slice_mut()[tail]))
        }
    }

    /// Get an element at the given index.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&A::Item> {
        if index < self.len() {
            self.ring_buffer
                .as_slice()
                .get(self.wrap_add(self.tail, index))
        } else {
            None
        }
    }

    /// Get a mutable reference to an element at a given index.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut A::Item> {
        if index < self.len() {
            let i = self.wrap_add(self.tail, index);
            self.ring_buffer.as_slice_mut().get_mut(i)
        } else {
            None
        }
    }

    /// Tell whether or not this `ArrayDeque` is contiguous.
    #[inline]
    pub fn is_contiguous(&self) -> bool {
        self.tail <= self.head
    }

    /// Get the contents of this `ArrayDeque` in the form of buffer slices.
    #[inline]
    pub fn as_slices(&self) -> (&[A::Item], &[A::Item]) {
        RingSlices::ring_slices(self.ring_buffer.as_slice(), self.head, self.tail)
    }

    /// Get the contents of this `ArrayDeque` in the form of mutable buffer slices.
    #[inline]
    pub fn as_mut_slices(&mut self) -> (&mut [A::Item], &mut [A::Item]) {
        RingSlices::ring_slices(self.ring_buffer.as_slice_mut(), self.head, self.tail)
    }

    /// Truncate this `ArrayDeque` to a certain size.
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        if len > self.len() {
            return;
        }

        let num_dropped = self.len() - len;
        let (front, back) = self.as_mut_slices();

        // see how many elements in the front slice we need to drop
        let front_dropped = core::cmp::min(num_dropped, front.len());
        front.iter_mut().take(front_dropped).for_each(|f| {
            mem::take(f);
        });

        // see how many elements in the back slice we need to drop
        let back_drop = num_dropped - front_dropped;
        back.iter_mut().take(back_drop).for_each(|b| {
            mem::take(b);
        });

        self.head = self.wrap_sub(self.head, num_dropped);
    }

    /// Clear this `ArrayDeque` of all elements.
    #[inline]
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Create a new iterator.
    #[inline]
    pub fn iter(&self) -> Iter<'_, A> {
        Iter {
            ring_buffer: self.ring_buffer.as_slice(),
            tail: self.tail,
            head: self.tail,
        }
    }
}

impl<A: Array> Clone for ArrayDeque<A>
where
    A::Item: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<A: Array> FromIterator<A::Item> for ArrayDeque<A> {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A::Item>,
    {
        let mut ad = ArrayDeque::new();
        ad.extend(iter);
        ad
    }
}

impl<A: Array> Extend<A::Item> for ArrayDeque<A> {
    #[inline]
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = A::Item>,
    {
        iter.into_iter().for_each(|item| self.push_back(item));
    }
}

/// An iterator over `ArrayDeque`s.
#[derive(Clone)]
pub struct Iter<'a, A: Array + 'a> {
    ring_buffer: &'a [A::Item],
    tail: usize,
    head: usize,
}

impl<'a, A: Array> Iterator for Iter<'a, A> {
    type Item = &'a A::Item;

    #[inline]
    fn next(&mut self) -> Option<&'a A::Item> {
        if self.tail == self.head {
            None
        } else {
            let tail = self.tail;
            self.tail =
                ArrayDeque::<A>::wrap_index(self.tail.wrapping_add(1), self.ring_buffer.len());
            Some(&self.ring_buffer[tail])
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = ArrayDeque::<A>::count(self.tail, self.head, self.ring_buffer.len());
        (len, Some(len))
    }
}

impl<'a, A: Array> DoubleEndedIterator for Iter<'a, A> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a A::Item> {
        if self.tail == self.head {
            None
        } else {
            self.head =
                ArrayDeque::<A>::wrap_index(self.head.wrapping_sub(1), self.ring_buffer.len());
            Some(&self.ring_buffer[self.head])
        }
    }
}

impl<'a, A: Array> ExactSizeIterator for Iter<'a, A> {}

impl<A: Array> FusedIterator for Iter<'_, A> {}

/*
/// A mutable iterator over an ArrayDeque.
pub struct IterMut<'a, A: Array + 'a> {
    ring_buffer: &'a mut [A::Item],
    tail: usize,
    head: usize,
}

impl<'a, A: Array> Iterator for IterMut<'a, A> {
    type Item = &'a mut A::Item;

    #[inline]
    fn next(&mut self) -> Option<&'a mut A::Item> {
        if self.tail == self.head {
            None
        } else {
            let tail = self.tail;
            self.tail =
                ArrayDeque::<A>::wrap_index(self.tail.wrapping_add(1), self.ring_buffer.len());
            self.ring_buffer.get_mut(tail)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = ArrayDeque::<A>::count(self.tail, self.head, self.ring_buffer.len());
        (len, Some(len))
    }
}

impl<'a, A: Array + 'a> DoubleEndedIterator for IterMut<'a, A> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut A::Item> {
        if self.tail == self.head {
            None
        } else {
            self.head =
                ArrayDeque::<A>::wrap_index(self.head.wrapping_sub(1), self.ring_buffer.len());
            self.ring_buffer.get_mut(self.head)
        }
    }
}

impl<'a, A: Array> ExactSizeIterator for IterMut<'a, A> {
}

impl<A: Array> FusedIterator for IterMut<'_, A> {}
*/

// ring slices
trait RingSlices: Sized {
    fn slice(self, from: usize, to: usize) -> Self;
    fn split_at(self, i: usize) -> (Self, Self);

    #[inline]
    fn ring_slices(buf: Self, head: usize, tail: usize) -> (Self, Self) {
        let contiguous = tail <= head;
        if contiguous {
            let (empty, buf) = buf.split_at(0);
            (buf.slice(tail, head), empty)
        } else {
            let (mid, right) = buf.split_at(tail);
            let (left, _) = mid.split_at(head);
            (right, left)
        }
    }
}

impl<T> RingSlices for &[T] {
    #[inline]
    fn slice(self, from: usize, to: usize) -> Self {
        &self[from..to]
    }
    #[inline]
    fn split_at(self, i: usize) -> (Self, Self) {
        (*self).split_at(i)
    }
}

impl<T> RingSlices for &mut [T] {
    #[inline]
    fn slice(self, from: usize, to: usize) -> Self {
        &mut self[from..to]
    }
    #[inline]
    fn split_at(self, i: usize) -> (Self, Self) {
        (*self).split_at_mut(i)
    }
}
