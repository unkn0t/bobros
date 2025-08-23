use core::arch::asm;

use crate::x86_64::DescriptorTablePointer;
use crate::x86_64::registers::SegmentSelector;

/// ## Safety
///
/// Caller must ensure that `gdt` points to the valid IDT and
/// that loading this IDT is safe.
pub unsafe fn lgdt(gdt: &DescriptorTablePointer) {
    unsafe {
        asm!("lgdt [{}]", in(reg) gdt, options(readonly, nostack, preserves_flags));
    }
}

/// ## Safety
///
/// Caller must ensure that `idt` points to the valid IDT and
/// that loading this IDT is safe.
pub unsafe fn lidt(idt: &DescriptorTablePointer) {
    unsafe {
        asm!("lidt [{}]", in(reg) idt, options(readonly, nostack, preserves_flags));
    }
}

/// ## Safety
///
/// This function is unsafe because the caller must ensure that the given
/// `selector` points to a valid TSS entry in the GDT and that the
/// corresponding data in the TSS is valid.
#[inline]
pub unsafe fn load_tss(selector: SegmentSelector) {
    unsafe {
        asm!("ltr {0:x}", in(reg) selector.0, options(nostack, preserves_flags));
    }
}

pub fn int3() {
    unsafe {
        asm!("int3", options(nomem, nostack));
    }
}
