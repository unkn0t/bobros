.section .multiboot_header
header_start:
        # IMPORTANT: flags must be aligned
        .align 8
        # magic number
        .long 0xE85250D6
        # architecture
        .long 0
        # header length
        .long header_end - header_start
        # checksum
        .long -(0xE85250D6 + 0 + (header_end - header_start))

        # optional multiboot tags

        # end tag
        .short 0
        .short 0
        .long  8
header_end:
