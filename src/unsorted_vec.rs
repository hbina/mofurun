use crate::sorted_vec::SortedVec;

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct UnsortedVec<T> {
    pub vec: Vec<T>,
}

impl<T> From<SortedVec<T>> for UnsortedVec<T> {
    fn from(x: SortedVec<T>) -> UnsortedVec<T> {
        UnsortedVec { vec: x.vec }
    }
}

impl<T> AsRef<[T]> for UnsortedVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.vec
    }
}

impl<T> IntoIterator for UnsortedVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<T> UnsortedVec<T> {
    pub fn from_sorted<V>(from: V) -> Self
    where
        V: Into<Vec<T>>,
    {
        Self { vec: from.into() }
    }

    /////////////////////////////////////////////
    /////
    ///// UnsortedVec API Mirror.
    /////
    /////////////////////////////////////////////

    pub fn push(mut self, item: T) -> Self {
        self.vec.push(item);
        self
    }

    pub fn sort(mut self) -> SortedVec<T>
    where
        T: Ord,
    {
        self.vec.sort();
        SortedVec::from(self)
    }
}
