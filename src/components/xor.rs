use crate::components::logic_gate::LogicGate;

pub struct XorGate {
    pub logic_gate: LogicGate,
}

impl XorGate {
    pub fn new(name: &str) -> Self {
        Self {
            logic_gate: LogicGate::new(name, 2, Self::op),
        }
    }

    pub fn load(&mut self, inputs: Vec<i32>) {
        self.logic_gate.load(inputs);
    }

    pub fn compute(&mut self) -> i32 {
        self.logic_gate.compute();
        self.logic_gate.out_val
    }

    pub fn represent(&self) -> String {
        self.logic_gate.represent()
    }

    fn op(inputs: Vec<i32>) -> i32 {
        inputs.iter().fold(0, |acc, &x| acc ^ x)
    }
}

pub struct XorGateN {
    pub logic_gate: LogicGate,
}

impl XorGateN {
    pub fn new(name: &str, size: i32) -> Self {
        Self {
            logic_gate: LogicGate::new(name, size, Self::op),
        }
    }

    pub fn load(&mut self, inputs: Vec<i32>) {
        self.logic_gate.load(inputs);
    }

    pub fn compute(&mut self) -> i32 {
        self.logic_gate.compute();
        self.logic_gate.out_val
    }

    pub fn represent(&self) -> String {
        self.logic_gate.represent()
    }

    fn op(inputs: Vec<i32>) -> i32 {
        inputs.iter().fold(0, |acc, &x| acc ^ x)
    }
}
