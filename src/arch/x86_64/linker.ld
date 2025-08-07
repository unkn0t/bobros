ENTRY(_start)

SECTIONS {
  . = 1M;

  .boot ALIGN(8) :
  {
    *(.multiboot_header)
  }

  .text ALIGN(16) :
  {
    *(.text)
  }

  .rodata ALIGN(8) :
  {
    *(.rodata)
  }

  .bss ALIGN(4096) :
  {
    *(.bss)
  }
}
