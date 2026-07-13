//! ESP32-S31 clock tree definitions and implementations.
//!
//! ## Overview
//!
//! The ESP32-S31 clock tree is derived from its `clocks.toml` metadata definition.
//! PLL helpers (80MHz, 160MHz, 240MHz) provide common clock sources for peripherals.
//!
//! **Note:** This is a minimal implementation stub. Full register-level clock
//! configuration will be completed when the esp32s31 PAC crate becomes available
//! from esp-rs/esp-pacs.

// TODO: This is a temporary place for this, should probably be moved into clocks_ll.

define_clock_tree_types!();

/// Clock configuration options.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(clippy::enum_variant_names, reason = "MHz suffix indicates physical unit.")]
#[non_exhaustive]
pub enum CpuClock {
    /// 80 MHz CPU clock
    #[default]
    _80MHz = 80,
    /// 160 MHz CPU clock
    _160MHz = 160,
    /// 240 MHz CPU clock
    _240MHz = 240,
}

impl CpuClock {
    const PRESET_80: ClockConfig = ClockConfig {
        xtal_clk: None,
        soc_root_clk: Some(SocRootClkConfig::Pll),
        cpu_hs_div: Some(CpuHsDivConfig::new(CpuHsDivDivisor::_1)),
        cpu_ls_div: None, // Unused when root clock is PLL
        ahb_hs_div: Some(AhbHsDivConfig::new(AhbHsDivDivisor::_3)),
        ahb_ls_div: None, // Unused when root clock is PLL
        mspi_fast_hs_clk: Some(MspiFastHsClkConfig::new(MspiFastHsClkDivisor::_5)),
        mspi_fast_ls_clk: None, // Unused when root clock is PLL
        apb_clk: Some(ApbClkConfig::new(ApbClkDivisor::_0)),
        ledc_sclk: Some(LedcSclkConfig::PllF80m),
        lp_fast_clk: None,
        lp_slow_clk: None,
        timg_calibration_clock: None,
    };
    const PRESET_160: ClockConfig = ClockConfig {
        xtal_clk: None,
        soc_root_clk: Some(SocRootClkConfig::Pll),
        cpu_hs_div: Some(CpuHsDivConfig::new(CpuHsDivDivisor::_0)),
        cpu_ls_div: None, // Unused when root clock is PLL
        ahb_hs_div: Some(AhbHsDivConfig::new(AhbHsDivDivisor::_3)),
        ahb_ls_div: None, // Unused when root clock is PLL
        mspi_fast_hs_clk: Some(MspiFastHsClkConfig::new(MspiFastHsClkDivisor::_5)),
        mspi_fast_ls_clk: None, // Unused when root clock is PLL
        apb_clk: Some(ApbClkConfig::new(ApbClkDivisor::_0)),
        ledc_sclk: Some(LedcSclkConfig::PllF80m),
        lp_fast_clk: None,
        lp_slow_clk: None,
        timg_calibration_clock: None,
    };
    const PRESET_240: ClockConfig = ClockConfig {
        xtal_clk: None,
        soc_root_clk: Some(SocRootClkConfig::Pll),
        cpu_hs_div: Some(CpuHsDivConfig::new(CpuHsDivDivisor::_3)),
        cpu_ls_div: None, // Unused when root clock is PLL
        ahb_hs_div: Some(AhbHsDivConfig::new(AhbHsDivDivisor::_7)),
        ahb_ls_div: None, // Unused when root clock is PLL
        mspi_fast_hs_clk: Some(MspiFastHsClkConfig::new(MspiFastHsClkDivisor::_5)),
        mspi_fast_ls_clk: None, // Unused when root clock is PLL
        apb_clk: Some(ApbClkConfig::new(ApbClkDivisor::_0)),
        ledc_sclk: Some(LedcSclkConfig::PllF80m),
        lp_fast_clk: None,
        lp_slow_clk: None,
        timg_calibration_clock: None,
    };
}

// Clock tree implementation stubs — these will be filled in with real register
// access once the esp32s31 PAC crate provides the clock register definitions.
// The infrastructure (ClockTree struct, refresh/enable functions) is generated
// by define_clock_tree_types!() from clocks.toml; only the *_impl functions
// need real implementations.

#[allow(unused_variables)]
pub fn configure_xtal_clk_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<XtalClkConfig>,
    _new_config: XtalClkConfig,
) {
    // Stub — actual implementation requires PAC register access
}

#[allow(unused_variables)]
pub fn enable_pll_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_rc_fast_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_xtal32k_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_osc_slow_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_rc_slow_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_hp_root_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_hp_root_clk_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<HpRootClkConfig>,
    _new_config: HpRootClkConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_cpu_clk_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<CpuClkConfig>,
    _new_config: CpuClkConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_ahb_clk_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<AhbClkConfig>,
    _new_config: AhbClkConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_mspi_fast_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_mspi_fast_clk_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<MspiFastClkConfig>,
    _new_config: MspiFastClkConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_soc_root_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_soc_root_clk_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<SocRootClkConfig>,
    _new_config: SocRootClkConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_cpu_hs_div_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_cpu_hs_div_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<CpuHsDivConfig>,
    _new_config: CpuHsDivConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_cpu_ls_div_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_cpu_ls_div_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<CpuLsDivConfig>,
    _new_config: CpuLsDivConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_ahb_hs_div_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_ahb_hs_div_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<AhbHsDivConfig>,
    _new_config: AhbHsDivConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_ahb_ls_div_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_ahb_ls_div_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<AhbLsDivConfig>,
    _new_config: AhbLsDivConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_apb_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_apb_clk_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<ApbClkConfig>,
    _new_config: ApbClkConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_mspi_fast_hs_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_mspi_fast_hs_clk_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<MspiFastHsClkConfig>,
    _new_config: MspiFastHsClkConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_mspi_fast_ls_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_mspi_fast_ls_clk_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<MspiFastLsClkConfig>,
    _new_config: MspiFastLsClkConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_pll_f80m_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub — S31 has 80MHz PLL as a fixed clock source
}

#[allow(unused_variables)]
pub fn enable_pll_f160m_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub — S31 has 160MHz PLL as a fixed clock source
}

#[allow(unused_variables)]
pub fn enable_pll_f240m_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub — S31 has 240MHz PLL as a fixed clock source
}

#[allow(unused_variables)]
pub fn enable_ledc_sclk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_ledc_sclk_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<LedcSclkConfig>,
    _new_config: LedcSclkConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_lp_fast_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_lp_slow_clk_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_lp_slow_clk_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<LpSlowClkConfig>,
    _new_config: LpSlowClkConfig,
) {
    // Stub
}

#[allow(unused_variables)]
pub fn enable_timg_calibration_clock_impl(_clocks: &mut ClockTree, _en: bool) {
    // Stub
}

#[allow(unused_variables)]
pub fn configure_timg_calibration_clock_impl(
    _clocks: &mut ClockTree,
    _old_config: Option<TimgCalibrationClockConfig>,
    _new_config: TimgCalibrationClockConfig,
) {
    // Stub
}
