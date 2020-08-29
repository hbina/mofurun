use crate::sorted_vec::SortedVec;

/// An unsorted variant of the vector.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct UnsortedVec<T>(pub(crate) Vec<T>);

impl<T> AsRef<[T]> for UnsortedVec<T> {
    /// Returns the reference.
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

/// [`UnsortedVec`] can be converted into an [`Iterator`].
impl<T> IntoIterator for UnsortedVec<T> {
    type Item = T;
    type IntoIter = UnsortedVecIterator<Self::Item>;

    /// Performs the conversion.
    fn into_iter(self) -> UnsortedVecIterator<T> {
        let result = UnsortedVecIterator {
            ptr: self.0.as_ptr(),
            len: self.0.len(),
            index: 0,
        };
        std::mem::forget(self);
        result
    }
}

/// Original functions.
impl<T> UnsortedVec<T> {
    /// Push an element.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mofurun::unsorted_vec::UnsortedVec;
    /// # pub fn main() {
    /// let unsorted = UnsortedVec::default().push(1);
    /// assert_eq!(unsorted.into_iter().next(), Some(1));
    /// # }
    /// ```
    ///
    pub fn push(mut self, item: T) -> Self {
        self.0.push(item);
        self
    }

    pub fn sort(mut self) -> SortedVec<T>
    where
        T: Ord,
    {
        self.0.sort();
        SortedVec(self.0)
    }
}

#[derive(Debug)]
pub struct UnsortedVecIterator<T> {
    pub(crate) ptr: *const T,
    pub(crate) len: usize,
    pub(crate) index: usize,
}

impl<Item> Iterator for UnsortedVecIterator<Item> {
    type Item = Item;

    /// Returns the current element and marches the iterator forward.
    /// This operation is O(1).
    ///
    /// # Example
    /// ```rust
    /// # use mofurun::unsorted_vec::UnsortedVec;
    /// # pub fn main() {
    /// assert_eq!(Some(1), UnsortedVec::default().push(1).push(3).push(5).push(7).push(44).into_iter().next());
    /// # }
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            let current = self.index;
            self.index += 1;
            unsafe { Some(std::ptr::read(self.ptr.add(current))) }
        }
    }

    /// Returns the size hint of this iterator.
    /// This operation is O(1).
    ///
    /// # Example
    /// ```rust
    /// # use mofurun::unsorted_vec::UnsortedVec;
    /// # pub fn main() {
    /// assert_eq!((5, Some(5)), UnsortedVec::default().push(1).push(3).push(5).push(7).push(44).into_iter().size_hint());
    /// # }
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    /// Returns the number of elements in this iterator.
    /// This operation is O(1).
    ///
    /// # Example
    /// ```rust
    /// # use mofurun::unsorted_vec::UnsortedVec;
    /// # pub fn main() {
    /// assert_eq!(5, UnsortedVec::default().push(1).push(3).push(5).push(7).push(44).into_iter().count());
    /// # }
    /// ```
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len
    }
}
