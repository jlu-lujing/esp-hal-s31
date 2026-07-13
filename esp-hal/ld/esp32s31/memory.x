/*
 * Memory layout for ESP32-S31
 * Based on ESP-IDF v6.2+ linker scripts
 */

MEMORY
{
    /*
     * Internal SRAM: 0x2F000000, ~477KB (after bootloader reservation)
     * The second stage bootloader loads at SRAM_SEG_END = 0x2F07AFC0
     * Available RAM: from SRAM_START to SRAM_SEG_END
     */
    RAM : ORIGIN = 0x2F000000, LENGTH = 0x7AFC0

    /*
     * PSRAM (external memory) - only needed if CONFIG_SPIRAM enabled.
     * In bare-metal RISC-V build without PSRAM, we don't map it here.
     */

    /*
     * LP RAM: used for RTC/fast memory on S31
     * The S31 uses a unified LP_RAM section (no separate fast/slow)
     */
    RTC_FAST : ORIGIN = 0x2E000000, LENGTH = 0x8000
}

REGION_ALIAS("iram_text_seg", RAM);
REGION_ALIAS("dram_seg", RAM);
REGION_ALIAS("rtc_text_seg", RTC_FAST);
REGION_ALIAS("rtc_data_seg", RTC_FAST);
REGION_ALIAS("rtc_fast_rwtext_seg", RTC_FAST);
REGION_ALIAS("rtc_fast_rwdata_seg", RTC_FAST);
