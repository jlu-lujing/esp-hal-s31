//! ESP32-S31 clock configuration and PLL helpers.

/// Return the 80 MHz PLL-derived clock frequency used as a common source.
pub fn pll_f80m_frequency() -> u32 {
    80_000_000
}

/// Return the 160 MHz PLL-derived clock frequency.
pub fn pll_f160m_frequency() -> u32 {
    160_000_000
}

/// Return the 240 MHz PLL-derived clock frequency.
pub fn pll_f240m_frequency() -> u32 {
    240_000_000
}
