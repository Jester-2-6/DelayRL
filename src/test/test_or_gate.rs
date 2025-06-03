use crate::components::or::OrGate;

#[test]
fn test_or_gate() {
    let mut gate = OrGate::new("OR Gate");
    gate.load(vec![0, 0]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![0, 1]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![1, 0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![1, 1]);
    assert_eq!(gate.compute(), 1);
}
