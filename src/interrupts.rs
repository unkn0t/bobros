use spin::Lazy;

use crate::x86_64::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{gdt, println};

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.double_fault.set_handler_fn(double_fault_handler);
    unsafe {
        idt.double_fault
            .options
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt
});

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    debug_assert_eq!(error_code, 0);
    panic!("EXCEPTION: DOUBLE FAULT\n{stack_frame:#?}");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT");
    println!("{stack_frame:#?}");
}
