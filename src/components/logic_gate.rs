use crate::common::constants::{DEFAULT_VALUE};
use crate::common::display::get_logic_level_display_name;
pub struct LogicGate {
    pub name: String,
    pub out_val: i32,
    pub size: i32,
    pub inputs: Vec<i32>,
    pub op: fn(Vec<i32>) -> i32, // Function to perform the logic operation
}

impl LogicGate {
    pub fn new(name: &str, size: i32, op: fn(Vec<i32>) -> i32) -> Self {
        LogicGate {
            name: name.to_string(),
            out_val: DEFAULT_VALUE,
            size,
            inputs: vec![DEFAULT_VALUE; size as usize],
            op,
        }
    }

    pub fn represent(&self) -> String {
        format!(
            "Logic Gate: {}, [{}] -> {}", 
            self.name, 
            self.inputs.iter().map(|&x| get_logic_level_display_name(x)).collect::<Vec<_>>().join(", "), 
            get_logic_level_display_name(self.out_val),
        )
    }

    pub fn load(&mut self, inputs: Vec<i32>) {
        if inputs.len() != self.size as usize {
            panic!("Input size does not match gate size");
        }
        self.inputs = inputs;
    }

    pub fn compute(&mut self) {
        self.out_val = (self.op)(self.inputs.clone());
    }

    pub fn reset(&mut self) {
        self.out_val = DEFAULT_VALUE;
        self.inputs.fill(DEFAULT_VALUE);
    }
}
