use crate::components::xor::XorGateN;

#[test]
fn test_xor_gate_n() {
    let mut gate = XorGateN::new("XOR Gate N", 3);
    gate.load(vec![0, 0, 0]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![0, 1, 0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![1, 0, 1]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![1, 1, 1]);
    assert_eq!(gate.compute(), 1);
}
