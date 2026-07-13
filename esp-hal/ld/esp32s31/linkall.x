/*
 * Root linker script for ESP32-S31
 * Based on esp32c6 pattern, adapted for S31-specific layout
 */

INCLUDE "memory.x"

REGION_ALIAS("ROTEXT", IRAM);
REGION_ALIAS("RWTEXT", RAM);
REGION_ALIAS("RODATA", RAM);
REGION_ALIAS("RWDATA", RAM);
REGION_ALIAS("RTC_FAST_RWTEXT", RTC_FAST);
REGION_ALIAS("RTC_FAST_RWDATA", RTC_FAST);

INCLUDE "esp32s31.x"
INCLUDE "hal-defaults.x"
