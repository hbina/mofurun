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

/////////////////////////////////////////////
/////
///// SortedVec API Mirror.
/////
/////////////////////////////////////////////
impl<T> SortedVec<T> {
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
