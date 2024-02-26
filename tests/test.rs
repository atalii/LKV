use lkv::*;

#[test]
fn otm_basics() {
    let mut c = OTM::new();
    c.insert(0, 0);

    assert_eq!(c.get(0), &[0]);

    let slice = c.get_mut(0);
    slice[0] = 1;

    assert_eq!(c.get(0), &[1]);

    c.insert(0, 0);

    assert_eq!(c.get(0), &[1, 0]);

    let mut o = OTM::new();
    o.insert(1, 1);

    c.merge(o);
    assert_eq!(c.get(0), &[1, 0]);
    assert_eq!(c.get(1), &[1]);
}
