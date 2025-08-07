ARCH ?= x86_64
KERNEL := build/kernel-$(ARCH).bin
ISO := build/bobros-$(ARCH).iso

LINKER_SCRIPT := src/arch/$(ARCH)/linker.ld
GRUB_CFG := src/arch/$(ARCH)/grub.cfg
ASM_SRCS := $(wildcard src/arch/$(ARCH)/*.asm)
ASM_OBJS := $(patsubst src/arch/$(ARCH)/%.asm, build/arch/$(ARCH)/%.o, $(ASM_SRCS))

TARGET ?= $(ARCH)-unknown-bobros
RUST_OS := target/$(TARGET)/debug/libbobros.a
	
.PHONY: all clean run run-dbg iso kernel

all: $(KERNEL)

clean:
	@cargo clean
	@rm -r build

run-dbg: $(ISO)
	@qemu-system-x86_64 -s -S -cdrom $(ISO)

run: $(ISO)
	@qemu-system-x86_64 -cdrom $(ISO)

iso: $(ISO)
	
$(ISO): $(KERNEL) $(GRUB_CFG)
	@mkdir -p build/isofiles/boot/grub
	@cp $(KERNEL) build/isofiles/boot/kernel.bin
	@cp $(GRUB_CFG) build/isofiles/boot/grub
	@grub-mkrescue -o $(ISO) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(KERNEL): kernel $(ASM_OBJS) $(LINKER_SCRIPT)
	@ld -n -T $(LINKER_SCRIPT) -o $(KERNEL) $(ASM_OBJS) $(RUST_OS)

kernel:
	@cargo build

build/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.asm
	@mkdir -pv $(shell dirname $@)
	@nasm -f elf64 $< -o $@


