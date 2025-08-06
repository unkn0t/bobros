ENTRY(_start)

SECTIONS {
  . = 1M;

  .boot ALIGN(8) :
  {
    *(.multiboot_header)
  }

  .text ALIGN(4096) :
  {
    *(.text)
  }
}
