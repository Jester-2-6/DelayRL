use crate::components::xor::XorGate;

#[test]
fn test_xor_gate() {
    let mut gate = XorGate::new("XOR Gate");
    gate.load(vec![0, 0]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![0, 1]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![1, 0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![1, 1]);
    assert_eq!(gate.compute(), 0);
}
