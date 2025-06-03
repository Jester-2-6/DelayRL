use crate::components::logic_gate::LogicGate;

pub struct NorGate {
    pub logic_gate: LogicGate,
}

impl NorGate {
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
        let mut result = 0;
        for &input in &inputs {
            result |= input;
        }
        (result == 0) as i32
    }
}

pub struct NorGateN {
    pub logic_gate: LogicGate,
}

impl NorGateN {
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
        let mut result = 0;
        for &input in &inputs {
            result |= input;
        }
        (result == 0) as i32
    }
}
