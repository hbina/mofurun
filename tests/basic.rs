use mofurun::{sorted_vec::SortedVec, unsorted_vec::UnsortedVec};

#[test]
pub fn basic_transformation_between_vec_types() {
    let s = SortedVec::default();
    let unsorted_s = s.push(5).push(4).push(3).push(2).push(1).push(0);
    let sorted = unsorted_s.sort();
    assert_eq!(vec![0, 1, 2, 3, 4, 5], sorted.as_ref());
}

#[test]
pub fn chaining_calls() {
    let s = SortedVec::default()
        .push(5)
        .push(4)
        .push(3)
        .push(2)
        .push(1)
        .push(0)
        .sort();
    assert_eq!(vec![0, 1, 2, 3, 4, 5], s.as_ref());
}

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

#[test]
pub fn zero_sized_type() {
    enum Ty {
        One,
    };
    assert_eq!(std::mem::size_of::<Ty>(), 0);
    let vec = vec![Ty::One, Ty::One, Ty::One, Ty::One, Ty::One];
    assert_eq!(
        UnsortedVec::new()
            .push(Ty::One)
            .push(Ty::One)
            .push(Ty::One)
            .push(Ty::One)
            .push(Ty::One)
            .into_iter()
            .fold(0, |acc, _| acc + 1),
        vec.into_iter().fold(0, |acc, _| acc + 1)
    );
    let lhs = vec![Ty::One, Ty::One, Ty::One, Ty::One, Ty::One];
    let rhs = vec![Ty::One, Ty::One, Ty::One, Ty::One, Ty::One];
    assert_eq!(
        lhs.into_iter().count(),
        rhs.into_iter().fold(0, |acc, _| acc + 1)
    );
}
