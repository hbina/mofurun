use crate::sorted_vec::SortedVec;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

/// An unsorted variant of the vector.
#[derive(Debug)]
pub struct UnsortedVec<T>(pub(crate) Vec<T>);

impl<T> Default for UnsortedVec<T>
where
    T: Default,
{
    fn default() -> UnsortedVec<T> {
        UnsortedVec(Vec::default())
    }
}

impl<T> PartialEq for UnsortedVec<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn ne(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }
}

impl<T> Eq for UnsortedVec<T> where T: PartialEq + Eq {}

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
        unsafe {
            let len = self.0.len();
            let cap = self.0.capacity();
            let mut me = ManuallyDrop::new(self.0);
            let begin = me.as_mut_ptr();
            let end = if std::mem::size_of::<T>() == 0 {
                std::intrinsics::arith_offset(begin as *const i8, len as isize) as *const T
            } else {
                begin.add(len) as *const T
            };
            let result = UnsortedVecIterator {
                buf: NonNull::new_unchecked(begin),
                ptr: begin,
                end,
                cap,
            };
            result
        }
    }
}

/// Original functions.
impl<T> UnsortedVec<T> {
    /// Creates a new empty [`UnsortedVec`].
    /// This is different than [`Default`] because it does not require `T` to be [`Default`].
    /// # Example
    ///
    /// ```rust
    /// use mofurun::unsorted_vec::UnsortedVec;
    /// pub fn main() {
    /// #[derive(Debug)]
    /// enum S {
    /// A,
    /// };
    /// let s = UnsortedVec::<S>::new();
    /// assert_eq!(s.len(), 0);
    /// println!("{:?}", s);
    /// }
    /// ```
    ///
    pub fn new() -> UnsortedVec<T> {
        UnsortedVec(Vec::new())
    }

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

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
pub struct UnsortedVecIterator<T> {
    pub(crate) buf: NonNull<T>,
    pub(crate) ptr: *const T,
    pub(crate) end: *const T,
    pub(crate) cap: usize,
}

impl<T> Iterator for UnsortedVecIterator<T> {
    type Item = T;

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
    fn next(&mut self) -> Option<T> {
        unsafe {
            if self.ptr as *const _ == self.end {
                None
            } else {
                if std::mem::size_of::<T>() == 0 {
                    // purposefully don't use 'ptr.offset' because for
                    // vectors with 0-size elements this would return the
                    // same pointer.
                    self.ptr = std::intrinsics::arith_offset(self.ptr as *const i8, 1) as *mut T;
                    // Make up a value of this ZST.
                    Some(std::mem::zeroed())
                } else {
                    let old = self.ptr;
                    self.ptr = self.ptr.offset(1);
                    Some(std::ptr::read(old))
                }
            }
        }
    }
}

impl<T> Drop for UnsortedVecIterator<T> {
    fn drop(&mut self) {
        if std::mem::size_of::<T>() != 0 {
            unsafe {
                let _ = Vec::from_raw_parts(
                    self.buf.as_ptr(),
                    self.end.offset_from(self.buf.as_ptr()) as usize,
                    self.cap,
                );
            }
        }
    }
}
