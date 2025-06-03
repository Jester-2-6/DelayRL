use crate::components::nor::NorGateN;

#[test]
fn test_nor_gate_n() {
    let mut gate = NorGateN::new("NOR Gate N", 3);
    gate.load(vec![0, 0, 0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![0, 1, 0]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![1, 0, 1]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![1, 1, 1]);
    assert_eq!(gate.compute(), 0);
}
