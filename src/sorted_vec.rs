use crate::unsorted_vec::UnsortedVec;

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct SortedVec<T> {
    pub vec: Vec<T>,
}

impl<T> From<UnsortedVec<T>> for SortedVec<T> {
    fn from(x: UnsortedVec<T>) -> SortedVec<T> {
        SortedVec { vec: x.vec }
    }
}

impl<T> AsRef<[T]> for SortedVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.vec
    }
}

impl<T> IntoIterator for SortedVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<T> SortedVec<T> {
    pub fn new() -> SortedVec<T> {
        SortedVec { vec: Vec::new() }
    }

    pub fn push(mut self, item: T) -> UnsortedVec<T> {
        self.vec.push(item);
        UnsortedVec::from(self)
    }

    pub fn sort(mut self) -> SortedVec<T>
    where
        T: Ord,
    {
        self.vec.sort();
        self
    }
}
