use crate::components::xnor::XnorGateN;

#[test]
fn test_xnor_gate_n() {
    let mut gate = XnorGateN::new("XNOR Gate N", 3);
    gate.load(vec![0, 0, 0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![0, 1, 0]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![1, 0, 1]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![1, 1, 1]);
    assert_eq!(gate.compute(), 0);
}
