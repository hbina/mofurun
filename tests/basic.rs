use mofurun::sorted_vec::SortedVec;

#[test]
pub fn basic_transformation_between_vec_types() {
    let mut s = SortedVec::default();
    let mut unsorted_s = s.push(4);
    unsorted_s = unsorted_s.push(3);
    unsorted_s = unsorted_s.push(2);
    unsorted_s = unsorted_s.push(5);
    unsorted_s = unsorted_s.push(8);
    unsorted_s = unsorted_s.push(4);
    println!("s:{:?}", unsorted_s);
    s = unsorted_s.sort();
    println!("s:{:?}", s);
}
