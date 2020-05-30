use crate::{sorted_vec::SortedVec, unsorted_vec::UnsortedVec};

// TODO: Add an additional member to specify _why_ it's currently in unsafe state.
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct UnsafeVec<T> {
    pub vec: Vec<T>,
}

impl<T> From<SortedVec<T>> for UnsafeVec<T> {
    fn from(x: SortedVec<T>) -> UnsafeVec<T> {
        UnsafeVec { vec: x.vec }
    }
}

impl<T> From<UnsortedVec<T>> for UnsafeVec<T> {
    fn from(x: UnsortedVec<T>) -> UnsafeVec<T> {
        UnsafeVec { vec: x.vec }
    }
}

impl<T> IntoIterator for UnsafeVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<T> UnsafeVec<T> {}
