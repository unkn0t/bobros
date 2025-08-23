use spin::Lazy;

use crate::x86_64::gdt::{Descriptor, GlobalDescriptorTable};
use crate::x86_64::instructions::load_tss;
use crate::x86_64::registers::{CS, SegmentReg, SegmentSelector};
use crate::x86_64::tss::TaskStateSegment;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 5 * 4096;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = &raw const STACK as u64;
        stack_start + STACK_SIZE as u64
    };
    tss
});

static GDT: Lazy<(GlobalDescriptorTable, Selectors)> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
    (
        gdt,
        Selectors {
            code_selector,
            tss_selector,
        },
    )
});

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    GDT.0.load();

    unsafe {
        CS::write(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}
