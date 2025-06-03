use crate::components::nand::NandGate;

#[test]
fn test_nand_gate() {
    let mut gate = NandGate::new("NAND Gate");
    gate.load(vec![0, 0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![0, 1]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![1, 0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![1, 1]);
    assert_eq!(gate.compute(), 0);
}
