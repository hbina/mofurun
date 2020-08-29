use crate::sorted_vec::SortedVec;
use std::cmp::Ordering;
use std::iter::{
    Chain, Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, FlatMap, Flatten, FromIterator,
    Fuse, Inspect, Map, Peekable, Product, Rev, Scan, Skip, SkipWhile, StepBy, Sum, Take,
    TakeWhile,
};

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

impl<T> std::convert::TryFrom<Vec<T>> for UnsortedVec<T>
where
    T: PartialEq + PartialOrd,
{
    type Error = ();

    /// Tries to [`UnsortedVector`] from a `Vec`.
    fn try_from(from: Vec<T>) -> Result<Self, ()> {
        if from.is_sorted() {
            Ok(Self(from))
        } else {
            Err(())
        }
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

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            let current = self.index;
            self.index += 1;
            unsafe { Some(std::ptr::read(self.ptr.add(current))) }
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
        unimplemented!()
    }

    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        unimplemented!()
    }

    fn step_by(self, step: usize) -> StepBy<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter>
    where
        Self: Sized,
        U: IntoIterator<Item = Self::Item>,
    {
        unimplemented!()
    }

    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        unimplemented!()
    }

    fn for_each<F>(self, f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        unimplemented!()
    }

    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        unimplemented!()
    }

    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        unimplemented!()
    }

    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn peekable(self) -> Peekable<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        unimplemented!()
    }

    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        unimplemented!()
    }

    fn skip(self, n: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
    where
        Self: Sized,
        F: FnMut(&mut St, Self::Item) -> Option<B>,
    {
        unimplemented!()
    }

    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
    where
        Self: Sized,
        U: IntoIterator,
        F: FnMut(Self::Item) -> U,
    {
        unimplemented!()
    }

    fn flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
        unimplemented!()
    }

    fn fuse(self) -> Fuse<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item),
    {
        unimplemented!()
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn partition<B, F>(self, f: F) -> (B, B)
    where
        Self: Sized,
        B: Default + Extend<Self::Item>,
        F: FnMut(&Self::Item) -> bool,
    {
        unimplemented!()
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        unimplemented!()
    }

    fn all<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        unimplemented!()
    }

    fn any<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        unimplemented!()
    }

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        unimplemented!()
    }

    fn find_map<B, F>(&mut self, f: F) -> Option<B>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        unimplemented!()
    }

    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    {
        unimplemented!()
    }

    fn rposition<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
        Self: Sized + ExactSizeIterator + DoubleEndedIterator,
    {
        unimplemented!()
    }

    fn max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        unimplemented!()
    }

    fn min(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        unimplemented!()
    }

    fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> B,
    {
        unimplemented!()
    }

    fn max_by<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        unimplemented!()
    }

    fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> B,
    {
        unimplemented!()
    }

    fn min_by<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        unimplemented!()
    }

    fn rev(self) -> Rev<Self>
    where
        Self: Sized + DoubleEndedIterator,
    {
        unimplemented!()
    }

    fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)
    where
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
        Self: Sized + Iterator<Item = (A, B)>,
    {
        unimplemented!()
    }

    fn copied<'a, T: 'a>(self) -> Copied<Self>
    where
        Self: Sized + Iterator<Item = &'a T>,
        T: Copy,
    {
        unimplemented!()
    }

    fn cloned<'a, T: 'a>(self) -> Cloned<Self>
    where
        Self: Sized + Iterator<Item = &'a T>,
        T: Clone,
    {
        unimplemented!()
    }

    fn cycle(self) -> Cycle<Self>
    where
        Self: Sized + Clone,
    {
        unimplemented!()
    }

    fn sum<S>(self) -> S
    where
        Self: Sized,
        S: Sum<Self::Item>,
    {
        unimplemented!()
    }

    fn product<P>(self) -> P
    where
        Self: Sized,
        P: Product<Self::Item>,
    {
        unimplemented!()
    }

    fn cmp<I>(self, other: I) -> Ordering
    where
        I: IntoIterator<Item = Self::Item>,
        Self::Item: Ord,
        Self: Sized,
    {
        unimplemented!()
    }

    fn partial_cmp<I>(self, other: I) -> Option<Ordering>
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    {
        unimplemented!()
    }

    fn eq<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<I::Item>,
        Self: Sized,
    {
        unimplemented!()
    }

    fn ne<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<I::Item>,
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
