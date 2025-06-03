use crate::components::buff::BuffGate;

#[test]
fn test_buff_gate() {
    let mut gate = BuffGate::new("BUFF Gate");
    gate.load(vec![0]);
    assert_eq!(gate.compute(), 0);

    gate.load(vec![1]);
    assert_eq!(gate.compute(), 1);
}
