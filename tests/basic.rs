use mofurun::sorted_vec::SortedVec;

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
