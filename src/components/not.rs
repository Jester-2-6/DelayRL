use crate::components::logic_gate::LogicGate;

pub struct NotGate {
    pub logic_gate: LogicGate,
}

impl NotGate {
    pub fn new(name: &str) -> Self {
        Self {
            logic_gate: LogicGate::new(name, 1, Self::op),
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
        (inputs[0] == 0) as i32
    }
}
