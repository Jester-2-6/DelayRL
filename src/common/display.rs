use crate::common::constants::LOGIC_DISPLAY_MAP;

/// Returns the display name for a given logic gate type.
pub fn get_logic_level_display_name<'a>(level: i32) -> &'a str {
    LOGIC_DISPLAY_MAP.get(level as usize).unwrap_or(&"Unknown")
}