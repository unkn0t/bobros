ENTRY(boot_entry)

PHDRS 
{
  text PT_LOAD FLAGS(5);
  data PT_LOAD FLAGS(6);
  bss PT_LOAD FLAGS(6);
}

SECTIONS {
  . = 1M;
  
  .boot :
  {
    *(.multiboot_header)
  } : text

  .text :
  {
    *(.text .text.*)
  } : text

  .rodata :
  {
    *(.srodata .srodata.*)
    *(.rodata .rodata.*)
  } : text

  .data :
  {
    *(.sdata .sdata.*)
    *(.data .data.*)
  } : data

  .bss :
  {
    . = ALIGN(4096);
    *(.sbss .sbss.*)
    . = ALIGN(4096);
    *(.bss .bss.*)
  } : bss
}
