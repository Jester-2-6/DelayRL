use crate::components::nor::NorGate;

#[test]
fn test_nor_gate() {
    let mut gate = NorGate::new("NOR Gate");
    gate.load(vec![0, 0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![0, 1]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![1, 0]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![1, 1]);
    assert_eq!(gate.compute(), 0);
}
