use mofurun::unsorted_vec::UnsortedVec;

pub fn main() {
    enum Ty {
        One,
    };

    fn create_vec() -> UnsortedVec<Ty> {
        UnsortedVec::new()
            .push(Ty::One)
            .push(Ty::One)
            .push(Ty::One)
            .push(Ty::One)
            .push(Ty::One)
    }

    assert_eq!(std::mem::size_of::<Ty>(), 0);
    let vec = vec![Ty::One, Ty::One, Ty::One, Ty::One, Ty::One];
    assert_eq!(create_vec().len(), vec.len());
    assert_eq!(
        create_vec().into_iter().fold(0, |acc, _| acc + 1),
        vec.into_iter().fold(0, |acc, _| acc + 1)
    );
    let lhs = create_vec();
    let rhs = create_vec();
    assert_eq!(
        lhs.into_iter().count(),
        rhs.into_iter().fold(0, |acc, _| acc + 1)
    );
}
