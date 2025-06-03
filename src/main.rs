mod test;
mod components;
mod common;

use components::and::AndGate;

fn simulate_circuit() {
    let mut and_gate = AndGate::new("a1");

    and_gate.load(vec![1, 0]); // Example inputs
    let result = and_gate.compute();
    println!("Result: {}", result);
    println!("Representation: {}", and_gate.represent());
}

fn main() {
    simulate_circuit();
}
