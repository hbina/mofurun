use mofurun::{sorted_vec::SortedVec, unsorted_vec::UnsortedVec};

#[test]
pub fn basic_iterator_test() {
    assert!(UnsortedVec::default()
        .push(1)
        .push(2)
        .push(3)
        .push(4)
        .push(5)
        .into_iter()
        .zip(
            SortedVec::default()
                .push(5)
                .push(4)
                .push(3)
                .push(2)
                .push(1)
                .sort()
                .into_iter()
        )
        .all(|(lhs, rhs)| lhs == rhs));
}
