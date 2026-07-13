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
     * Memory available after the 2nd stage bootloader is finished.
     * Maps the tail of SRAM for .dram2_uninit section.
     */
    dram2_seg ( RW ) : ORIGIN = ORIGIN(RAM) + LENGTH(RAM), LENGTH = 0x20

    /*
     * PSRAM (external memory) - only needed if CONFIG_SPIRAM enabled.
     * In bare-metal RISC-V build without PSRAM, we don't map it here.
     */

    /*
     * LP RAM: used for RTC/fast memory on S31
     * The S31 uses a unified LP_RAM section (no separate fast/slow)
     */
    RTC_FAST : ORIGIN = 0x2E000000, LENGTH = 0x8000

    /*
     * Instruction and Data ROM (external flash mapped via cache)
     * S31 supports up to 4MB flash. The 0x20 offset is a convenience for the
     * app binary image generation: flash cache has 64KB pages, and each segment
     * has an 0x08 byte header.
     */
    ROM : ORIGIN =   0x38000020, LENGTH = 0x400000 - 0x20

    /*
     * External RAM (PSRAM) — optional. The Wokwi S31 simulator uses a
     * Function Core board which may or may not have PSRAM. This region is a
     * placeholder: set ORIGIN/LENGTH to match your actual hardware config.
     */
    EXTERNAL_RAM ( RW ) : ORIGIN = 0x4002_0000, LENGTH = 0
}

REGION_ALIAS("iram_text_seg", RAM);
REGION_ALIAS("dram_seg", RAM);
REGION_ALIAS("rtc_text_seg", RTC_FAST);
REGION_ALIAS("rtc_data_seg", RTC_FAST);
REGION_ALIAS("rtc_fast_rwtext_seg", RTC_FAST);
REGION_ALIAS("rtc_fast_rwdata_seg", RTC_FAST);
