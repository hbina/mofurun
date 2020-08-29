use crate::unsorted_vec::UnsortedVec;
use std::cmp::Ordering;
use std::iter::StepBy;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SortedVec<T>(pub(crate) Vec<T>);

impl<T> AsRef<[T]> for SortedVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T> IntoIterator for SortedVec<T> {
    type Item = T;
    type IntoIter = SortedVecIterator<Self::Item>;

    fn into_iter(self) -> SortedVecIterator<T> {
        let result = SortedVecIterator {
            ptr: self.0.as_ptr(),
            len: self.0.len(),
            index: 0,
        };
        std::mem::forget(self);
        result
    }
}

impl<T> std::convert::TryFrom<Vec<T>> for SortedVec<T>
where
    T: PartialEq + PartialOrd,
{
    type Error = &'static str;

    /// Tries to create a [`SortedVec`] from a `Vec`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mofurun::sorted_vec::SortedVec;
    /// # use std::convert::TryFrom;
    /// # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sorted : SortedVec<i32> = SortedVec::try_from(vec![1,2,3,4,5,6])?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn try_from(from: Vec<T>) -> Result<SortedVec<T>, &'static str> {
        if from.is_sorted() {
            Ok(SortedVec(from))
        } else {
            Err("attempting to perform a conversion from a non-sorted `Vec` to `SortedVector`.")
        }
    }
}

/// Original functions.
impl<T> SortedVec<T> {
    /// Pushes an element into the vector and returns an [`UnsortedVector`] variant of this vector.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mofurun::{sorted_vec::SortedVec, unsorted_vec::UnsortedVec};
    /// # pub fn main() {
    /// let mut sorted = SortedVec::default().push(1).push(2).push(3).push(4).push(5).sort().into_iter();
    /// assert_eq!(sorted.next(), Some(1));
    /// assert_eq!(sorted.next(), Some(2));
    /// assert_eq!(sorted.next(), Some(3));
    /// assert_eq!(sorted.next(), Some(4));
    /// assert_eq!(sorted.next(), Some(5));
    /// # }
    /// ```
    ///
    pub fn push(mut self, item: T) -> UnsortedVec<T> {
        self.0.push(item);
        UnsortedVec(self.0)
    }
}

#[derive(Debug)]
pub struct SortedVecIterator<T> {
    pub(crate) ptr: *const T,
    pub(crate) len: usize,
    pub(crate) index: usize,
}

impl<Ty> Iterator for SortedVecIterator<Ty> {
    type Item = Ty;

    /// Returns the current element and marches the iterator forward.
    /// This operation is O(1).
    ///
    /// # Example
    /// ```rust
    /// # use mofurun::sorted_vec::SortedVec;
    /// # pub fn main() {
    /// assert_eq!(Some(1), SortedVec::default().push(1).push(3).push(5).push(7).push(44).into_iter().next());
    /// # }
    /// ```
    fn next(&mut self) -> Option<Ty> {
        if self.index >= self.len {
            None
        } else {
            let current = self.index;
            self.index += 1;
            Some(unsafe { std::ptr::read(self.ptr.add(current)) })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    /// Returns the number of elements inside this iterator.
    /// This operation is O(1).
    ///
    /// # Example
    /// ```rust
    /// # use mofurun::sorted_vec::SortedVec;
    /// # pub fn main() {
    /// assert_eq!(5, SortedVec::default().push(1).push(3).push(5).push(7).push(44).into_iter().count());
    /// # }
    /// ```
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len
    }

    /// Returns the last element inside a [`SortedVector`].
    /// This operation is O(1).
    ///
    /// # Example
    /// ```rust
    /// # use mofurun::sorted_vec::SortedVec;
    /// # pub fn main() {
    /// assert_eq!(Some(44), SortedVec::default().push(1).push(3).push(5).push(7).push(44).into_iter().last());
    /// # }
    /// ```
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.len == 0 {
            None
        } else {
            unsafe { Some(std::ptr::read(self.ptr.add(self.len - 1))) }
        }
    }

    /// Returns the nth element inside a [`SortedVector`].
    /// This operation is O(1).
    ///
    /// # Example
    /// ```rust
    /// # use mofurun::sorted_vec::SortedVec;
    /// # pub fn main() {
    /// assert_eq!(Some(7), SortedVec::default().push(1).push(3).push(5).push(7).push(10).into_iter().nth(3));
    /// # }
    /// ```
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len {
            None
        } else {
            unsafe { Some(std::ptr::read(self.ptr.add(n))) }
        }
    }

    /// Returns the max element inside a [`SortedVector`].
    /// This operation is O(1).
    ///
    /// # Example
    /// ```rust
    /// # use mofurun::sorted_vec::SortedVec;
    /// # pub fn main() {
    /// assert_eq!(Some(10), SortedVec::default().push(1).push(3).push(5).push(7).push(10).into_iter().max());
    /// # }
    /// ```
    fn max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        if self.len == 0 {
            None
        } else {
            unsafe { Some(std::ptr::read(self.ptr.add(self.len - 1))) }
        }
    }

    /// Returns the min element inside a [`SortedVector`].
    /// This operation is O(1).
    ///
    /// # Example
    /// ```rust
    /// # use mofurun::sorted_vec::SortedVec;
    /// # pub fn main() {
    /// assert_eq!(Some(1), SortedVec::default().push(1).push(3).push(5).push(7).push(10).into_iter().min());
    /// # }
    /// ```
    fn min(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        if self.len == 0 {
            None
        } else {
            unsafe { Some(std::ptr::read(self.ptr.add(0))) }
        }
    }
}
