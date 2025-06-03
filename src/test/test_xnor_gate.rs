use crate::components::xnor::XnorGate;

#[test]
fn test_xnor_gate() {
    let mut gate = XnorGate::new("XNOR Gate");
    gate.load(vec![0, 0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![0, 1]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![1, 0]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![1, 1]);
    assert_eq!(gate.compute(), 1);
}
