//! ESP32-S31 GPIO configuration.

/// Returns true if the given address is a valid GPIO pad address for ESP32-S31.
pub fn is_valid_gpio_address(address: usize) -> bool {
    // GPIO register region
    (0x2058_3000..=0x2058_3FFF).contains(&(address as u32))
}
