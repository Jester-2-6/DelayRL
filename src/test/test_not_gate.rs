use crate::components::not::NotGate;

#[test]
fn test_not_gate() {
    let mut gate = NotGate::new("NOT Gate");
    gate.load(vec![0]);
    assert_eq!(gate.compute(), 1);

    gate.load(vec![1]);
    assert_eq!(gate.compute(), 0);
}
