use mofurun::{sorted_vec::SortedVec, unsorted_vec::UnsortedVec};

#[test]
pub fn basic_iterator_test() {
    let sorted = SortedVec::default()
        .push(5)
        .push(4)
        .push(3)
        .push(2)
        .push(1)
        .push(0)
        .sort()
        .into_iter();
    UnsortedVec::default()
        .push(0)
        .push(1)
        .push(2)
        .push(3)
        .push(4)
        .push(5)
        .into_iter()
        .zip(sorted)
        .for_each(|(x, y)| assert_eq!(x, y));
}
