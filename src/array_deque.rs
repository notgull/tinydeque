// MIT/Apache2 License

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
#[derive(Debug)]
pub struct ArrayDeque<A: Array> {
    ring_buffer: A,
    head: usize,
    tail: usize,
    len: usize,
}

impl<A: Array> Default for ArrayDeque<A> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
fn wrap_index(index: isize, size: isize) -> usize {
    let base = index % size;
    if base < 0 {
        (size + base) as usize
    } else {
        base as usize
    }
}

#[inline]
fn wrap_add(index: usize, add: usize, size: usize) -> usize {
    wrap_index((index as isize).wrapping_add(add as isize), size as isize)
}

#[inline]
fn wrap_sub(index: usize, sub: usize, size: usize) -> usize {
    wrap_index((index as isize).wrapping_sub(sub as isize), size as isize)
}

impl<A: Array> ArrayDeque<A> {
    /// Create a new `ArrayDeque`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tinydeque::ArrayDeque;
    /// let foobar: ArrayDeque<[i32; 5]> = ArrayDeque::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            ring_buffer: A::default(),
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    /// The capacity of this `ArrayDeque`. This is the maximum number of elements that can be
    /// stored in this `ArrayDeque`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tinydeque::ArrayDeque;
    /// assert_eq!(ArrayDeque::<[&'static str; 8]>::capacity(), 8);
    /// ```
    #[inline]
    #[must_use]
    pub fn capacity() -> usize {
        A::CAPACITY
    }

    /// Helper function to get len.
    #[inline]
    fn count(tail: usize, head: usize, size: usize) -> usize {
        wrap_sub(head, tail, size)
    }

    /// Get the length of this `ArrayDeque`.
    ///
    /// # Example
    ///
    /// ```
    /// use tinydeque::ArrayDeque;
    ///
    /// // we've been hired by the Crab Patrol to find and destroy some crabs
    /// // they said we shouldn't use Rust to do this but let's do it anyways
    ///
    /// /// Representative of a single crab.
    /// #[derive(Default)]
    /// struct Crab {
    ///     diameter: u16,
    ///     danger_level: u16,
    /// }
    ///
    /// let mut crab_hitlist: ArrayDeque<[Crab; 10]> = ArrayDeque::new();
    /// for i in 1..=10 {
    ///     crab_hitlist.push_back(Crab { diameter: i, danger_level: 100 / i }); // small crabs are more dangerous
    /// }
    ///
    /// assert_eq!(crab_hitlist.len(), 10);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Tell whether this `ArrayDeque` is empty.
    ///
    /// # Example
    ///
    /// ```
    /// # use tinydeque::ArrayDeque;
    /// let empty_deque: ArrayDeque<[(); 12]> = ArrayDeque::new();
    /// assert!(empty_deque.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Tell whether this `ArrayDeque` is full, or its entire capacity is filled with elements.
    ///
    /// # Example
    ///
    /// ```
    /// # use tinydeque::ArrayDeque;
    /// let full_deque: ArrayDeque<[i64; 12]> = (0i64..12).into_iter().collect();
    /// assert!(full_deque.is_full());
    /// ```
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == Self::capacity()
    }

    /// Push an element onto the back of this `ArrayDeque`.
    ///
    /// # Errors
    ///
    /// If this `ArrayDeque` is full, this function returns an Err with the rejected element.
    ///
    /// # Example
    ///
    /// ```
    /// use tinydeque::ArrayDeque;
    ///
    /// // we've been hired by the United Artists of America to manage their art gallery
    /// // because they're starving artists, they don't have the money to afford good hardware
    /// // thus we can only store 5 paintings at a time
    ///
    /// /// Represents a painting.
    /// #[derive(Default)]
    /// struct Painting {
    ///     name: &'static str,
    ///     rating: u8,
    /// }
    ///
    /// let mut painting_list: ArrayDeque<[Painting; 10]> = ArrayDeque::new();
    /// let mut i = 0;
    ///
    /// // we have a lot of paintings named "The Jaguar" of questionable quality
    /// while let Ok(()) = painting_list.try_push_back(Painting { name: "The Jaguar", rating: 3 }) { i += 1; }
    ///
    /// assert_eq!(i, 10);
    /// ```
    #[inline]
    pub fn try_push_back(&mut self, element: A::Item) -> Result<(), A::Item> {
        // if we're full, error out
        if self.is_full() {
            return Err(element);
        }

        // increment the head by one
        let head = self.head;
        self.head = wrap_add(self.head, 1, Self::capacity());
        self.ring_buffer.as_slice_mut()[head] = element;
        self.len += 1;
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

        self.tail = wrap_sub(self.tail, 1, Self::capacity());
        self.ring_buffer.as_slice_mut()[self.tail] = element;
        self.len += 1;
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
            self.head = wrap_sub(self.head, 1, Self::capacity());
            self.len -= 1;
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
            self.tail = wrap_add(self.tail, 1, Self::capacity());
            self.len -= 1;
            Some(mem::take(&mut self.ring_buffer.as_slice_mut()[tail]))
        }
    }

    /// Get an element at the given index.
    ///
    /// # Example
    ///
    /// ```
    /// use tinydeque::ArrayDeque;
    ///
    /// let mut my_favorite_numbers = ArrayDeque::<[i32; 6]>::new();
    /// my_favorite_numbers.push_back(5);
    /// my_favorite_numbers.push_back(50);
    /// my_favorite_numbers.push_back(33);
    /// my_favorite_numbers.push_front(48);
    ///
    /// assert_eq!(my_favorite_numbers.get(0), Some(&48));
    /// assert_eq!(my_favorite_numbers.get(2), Some(&50));
    /// assert_eq!(my_favorite_numbers.get(4), None);
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<&A::Item> {
        if index < self.len() {
            self.ring_buffer
                .as_slice()
                .get(wrap_add(self.tail, index, Self::capacity()))
        } else {
            None
        }
    }

    /// Get a mutable reference to an element at a given index.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut A::Item> {
        if index < self.len() {
            let i = wrap_add(self.tail, index, Self::capacity());
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

        self.head = wrap_sub(self.head, num_dropped, Self::capacity());
    }

    /// Clear this `ArrayDeque` of all elements.
    #[inline]
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Create a new iterator.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &A::Item> {
        let (front, back) = self.as_slices();
        front.iter().chain(back.iter())
    }

    /// Create an new mutable iterator.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut A::Item> {
        let (front, back) = self.as_mut_slices();
        front.iter_mut().chain(back.iter_mut())
    }

    /// Append another `ArrayDeque` onto the back of one.
    ///
    /// # Errors
    ///
    /// If the `ArrayDeque`'s contents cannot fit into this one, the Err value is returned.
    #[inline]
    pub fn append(&mut self, other: &mut Self) -> Result<(), ()> {
        if self.len() + other.len() > Self::capacity() {
            Err(())
        } else {
            while let Some(item) = other.pop_front() {
                self.push_back(item);
            }
            Ok(())
        }
    }

    /// Get the back item of this `ArrayDeque`.
    #[inline]
    pub fn back(&self) -> Option<&A::Item> {
        self.get(self.len - 1)
    }

    /// Get a mutable reference to the back item of this `ArrayDeque`.
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut A::Item> {
        self.get_mut(self.len - 1)
    }

    /// Get the front item of this `ArrayDeque`.
    #[inline]
    pub fn front(&self) -> Option<&A::Item> {
        self.get(0)
    }

    /// Get a mutable reference to the front item of this `ArrayDeque`.
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut A::Item> {
        self.get_mut(0)
    }

    /// Tell whether or not this deque contains an element.
    #[inline]
    pub fn contains(&self, item: &A::Item) -> bool
    where
        A::Item: PartialEq,
    {
        let (front, back) = self.as_slices();
        front.contains(item) || back.contains(item)
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
            self.tail = wrap_add(self.tail, 1, self.ring_buffer.len());
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
            self.head = wrap_sub(self.head, 1, self.ring_buffer.len());
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

#[test]
fn test_index_wrap() {
    assert_eq!(wrap_index(1, 10), 1);
    assert_eq!(wrap_index(13, 10), 3);
    assert_eq!(wrap_index(10, 10), 0);

    assert_eq!(wrap_add(9, 2, 10), 1);
    assert_eq!(wrap_add(5, 2, 10), 7);

    assert_eq!(wrap_sub(1, 6, 10), 5, "subtraction test");
}
