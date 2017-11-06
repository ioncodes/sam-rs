#![feature(plugin)]
#![plugin(sam)]

#[test]
fn test_sam() {
    assert_eq!("b83000", sam!("mov eax, 3"));
}
