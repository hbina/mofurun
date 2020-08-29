use crate::unsorted_vec::UnsortedVec;
use std::cmp::Ordering;
use std::iter::{Cloned, Copied, Cycle, Product, Rev, Sum};

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

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len
    }

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

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len {
            None
        } else {
            unsafe { Some(std::ptr::read(self.ptr.add(n))) }
        }
    }

    fn max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.max()
    }

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

    fn partial_cmp<I>(self, other: I) -> Option<Ordering>
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    {
        unimplemented!()
    }

    fn lt<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    {
        unimplemented!()
    }

    fn le<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    {
        unimplemented!()
    }

    fn gt<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    {
        unimplemented!()
    }

    fn ge<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    {
        unimplemented!()
    }
}
