#[cfg(test)]
mod tests {
    use crate::components::and::{AndGate, AndGateN};

    #[test]
    fn test_and_gate_basic() {
        let mut and_gate = AndGate::new("TestGate");
        and_gate.load(vec![1, 1]);
        assert_eq!(and_gate.compute(), 1);

        and_gate.load(vec![1, 0]);
        assert_eq!(and_gate.compute(), 0);

        and_gate.load(vec![0, 0]);
        assert_eq!(and_gate.compute(), 0);
    }

    #[test]
    fn test_and_gate_3_basic() {
        let mut and_gate_n = AndGateN::new("TestGateN", 3);
        and_gate_n.load(vec![1, 1, 1]);
        assert_eq!(and_gate_n.compute(), 1);

        and_gate_n.load(vec![1, 0, 1]);
        assert_eq!(and_gate_n.compute(), 0);

        and_gate_n.load(vec![0, 0, 0]);
        assert_eq!(and_gate_n.compute(), 0);
    }

    #[test]
    fn test_and_gate_4_basic() {
        let mut and_gate_n = AndGateN::new("TestGateN", 4);
        and_gate_n.load(vec![1, 1, 1, 1]);
        assert_eq!(and_gate_n.compute(), 1);
        and_gate_n.load(vec![1, 0, 1, 1]);
        assert_eq!(and_gate_n.compute(), 0);
        and_gate_n.load(vec![0, 0, 0, 0]);
        assert_eq!(and_gate_n.compute(), 0);
    }
}
