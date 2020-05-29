use mofurun::Mofu;

#[test]
pub fn basic() {
    let mut s = Mofu::default();
    s = s.push(4);
    s = s.push(3);
    s = s.push(2);
    s = s.push(5);
    s = s.push(8);
    s = s.push(4);
    println!("s:{:?}", s);
    s = s.sort();
    println!("s:{:?}", s);
}
