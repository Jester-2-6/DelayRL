/// Logic level constants used throughout the application.
/// 
/// - `L_0`: Represents logic level 0.
/// - `L_1`: Represents logic level 1.
/// - `L_X`: Represents an unknown or undefined logic level.
/// - `L_RISE`: Represents a rising edge in logic level.
/// - `L_FALL`: Represents a falling edge in logic level.
/// - `L_D`: Represents the D (detect) logic value, often used in test pattern generation.
/// - `L_DBAR`: Represents the complement of D (D-bar) logic value.
///
/// Other constants:
/// - `DEFAULT_VALUE`: The default value for logic gates.
/// - `AND`: The display name for an AND gate.
/// - `OR`: The display name for an OR gate.
pub const L_0: i32 = 0;
pub const L_1: i32 = 1;
pub const L_X: i32 = 2;
pub const L_RISE: i32 = 3;
pub const L_FALL: i32 = 4;
pub const L_D: i32 = 5;
pub const L_DBAR: i32 = 6;

pub const DEFAULT_VALUE: i32 = L_X;
pub const DEFAULT_SIZE: i32 = 2;
pub const LOGIC_DISPLAY_MAP: [&str; 7] = [
    "0",   // 0
    "1",   // 1
    "X",   // 2
    "RISE",// 3
    "FALL",// 4
    "D",   // 5
    "DB"   // 6
];