/*
 * ESP32-S31 specific linker fixups
 * Based on ESP-IDF master + RISC-V target requirements
 */

/*
 * ESP32-S31 is a dual-core RISC-V SoC, so we need hart IDs 0 and 1.
 */
PROVIDE(_max_hart_id = 1);

/* Shared sections - ordering matters */
SECTIONS {
  INCLUDE "rwtext.x"
  INCLUDE "rwdata.x"
}
/* End of Shared sections */

SECTIONS {
  /**
   * Bootloader requires separate segments for ROTEXT and RODATA.
   * Create a gap between them to satisfy MMU constraints.
   */
  .text_gap (NOLOAD): {
    . = . + 8;
    . = ALIGN(0x10000) + 0x20;
  } > RAM
}
INSERT BEFORE .text;

/* Shared sections #2 - ordering matters */
INCLUDE "rodata.x"
INCLUDE "text.x"
INCLUDE "rtc_fast.x"
INCLUDE "stack.x"
INCLUDE "dram2.x"
INCLUDE "metadata.x"
INCLUDE "eh_frame.x"
/* End of Shared sections #2 */

_dram_data_start = ORIGIN(RAM) + SIZEOF(.trap) + SIZEOF(.rwtext);
