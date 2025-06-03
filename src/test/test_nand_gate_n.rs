use crate::components::nand::NandGateN;

#[test]
fn test_nand_gate_n() {
    let mut gate = NandGateN::new("NAND Gate N", 3);
    gate.load(vec![0, 0, 0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![0, 1, 0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![1, 0, 1]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![1, 1, 1]);
    assert_eq!(gate.compute(), 0);
}
