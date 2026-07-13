//! # SOC (System-on-Chip) module (ESP32-S31)
//!
//! ## Overview
//!
//! The `SOC` module provides access, functions and structures that are useful
//! for interacting with various system-related peripherals on `ESP32-S31` chip.
//!
//! The ESP32-S31 is a dual-core RISC-V SoC running at up to 320MHz.

crate::unstable_module! {
    pub mod clocks;
}
pub mod gpio;

// Re-export the PAC crate when available from esp-rs/esp-pacs.
// Uncomment once the PAC exists: `#[cfg(feature = "esp32s31")] pub(crate) use esp32s31 as pac;

#[cfg(i2s_driver_supported)]
#[cfg_attr(not(feature = "unstable"), allow(unused))]
pub(crate) fn i2s_sclk_frequency() -> u32 {
    clocks::pll_f80m_frequency()
}

#[cfg(feature = "rt")]
pub(crate) fn riscv_preinit() {}

pub(crate) fn pre_init() {
    // Disable APM access path filters early.
    // These are enabled by default and only allow TEE-mode access.
    // Since HP CPU boots in REE mode, these must be disabled.

    #[cfg(apu_driver_supported)]
    crate::peripherals::LP_APM::regs()
        .func_ctrl()
        .write(|w| unsafe { w.bits(0x0) });
    #[cfg(apu_driver_supported)]
    crate::peripherals::HP_APM::regs()
        .func_ctrl()
        .write(|w| unsafe { w.bits(0x0) });
}
