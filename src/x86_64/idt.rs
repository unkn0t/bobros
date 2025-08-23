use core::fmt;
use core::marker::PhantomData;

use bit_field::BitField;

use crate::x86_64::DescriptorTablePointer;
use crate::x86_64::registers::{CS, SegmentReg, SegmentSelector};
use crate::x86_64::rflags::RFlags;

// https://wiki.osdev.org/Exceptions
#[repr(C, align(16))]
pub struct InterruptDescriptorTable {
    pub divide_error: Entry<HandlerFunc>,
    pub debug: Entry<HandlerFunc>,
    pub non_maskable_interrupt: Entry<HandlerFunc>,
    pub breakpoint: Entry<HandlerFunc>,
    pub overflow: Entry<HandlerFunc>,
    pub bound_range_exceeded: Entry<HandlerFunc>,
    pub invalid_opcode: Entry<HandlerFunc>,
    pub device_not_available: Entry<HandlerFunc>,
    pub double_fault: Entry<DivergingHandlerFuncWithErr>,
    coprocessor_segment_overrun: Entry<HandlerFunc>,
    pub invalid_tss: Entry<HandlerFuncWithErr>,
    pub segment_not_present: Entry<HandlerFuncWithErr>,
    pub stack_segment_fault: Entry<HandlerFuncWithErr>,
    pub general_protection_fault: Entry<HandlerFuncWithErr>,
    pub page_fault: Entry<HandlerFuncWithErr>,
    reserved1: Entry<HandlerFunc>,
    pub x87_floating_point: Entry<HandlerFunc>,
    pub alignment_check: Entry<HandlerFuncWithErr>,
    pub machine_check: Entry<DivergingHandlerFunc>,
    pub simd_floating_point: Entry<HandlerFunc>,
    pub virtualization: Entry<HandlerFunc>,
    pub cp_protection_exception: Entry<HandlerFuncWithErr>,
    reserved2: [Entry<HandlerFunc>; 6],
    pub hv_injection_exception: Entry<HandlerFunc>,
    pub vmm_communication_exception: Entry<HandlerFuncWithErr>,
    pub security_exception: Entry<HandlerFuncWithErr>,
    reserved3: Entry<HandlerFunc>,
    interrupts: [Entry<HandlerFunc>; 256 - 32],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Entry<F> {
    ptr_low: u16,
    pub options: EntryOptions,
    ptr_mid: u16,
    ptr_high: u32,
    reserved: u32,
    phantom: PhantomData<F>,
}
assert_eq_size!(Entry<HandlerFunc>, [u8; 16]);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EntryOptions {
    code_selector: SegmentSelector,
    bits: u16,
}

/// # Safety
/// Implementors have to ensure `to_addr` returns a valid address.
pub unsafe trait InterruptHandlerFn {
    fn to_addr(self) -> u64;
}

type HandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame);
type HandlerFuncWithErr = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
type DivergingHandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame) -> !;
type DivergingHandlerFuncWithErr =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64) -> !;

#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: SegmentSelector,
    _reserved1: [u8; 6],
    pub cpu_flags: RFlags,
    pub stack_pointer: u64,
    pub stack_segment: SegmentSelector,
    _reserved2: [u8; 6],
}

impl InterruptDescriptorTable {
    pub const fn new() -> Self {
        Self {
            divide_error: Entry::missing(),
            debug: Entry::missing(),
            non_maskable_interrupt: Entry::missing(),
            breakpoint: Entry::missing(),
            overflow: Entry::missing(),
            bound_range_exceeded: Entry::missing(),
            invalid_opcode: Entry::missing(),
            device_not_available: Entry::missing(),
            double_fault: Entry::missing(),
            coprocessor_segment_overrun: Entry::missing(),
            invalid_tss: Entry::missing(),
            segment_not_present: Entry::missing(),
            stack_segment_fault: Entry::missing(),
            general_protection_fault: Entry::missing(),
            page_fault: Entry::missing(),
            reserved1: Entry::missing(),
            x87_floating_point: Entry::missing(),
            alignment_check: Entry::missing(),
            machine_check: Entry::missing(),
            simd_floating_point: Entry::missing(),
            virtualization: Entry::missing(),
            cp_protection_exception: Entry::missing(),
            reserved2: [Entry::missing(); 6],
            hv_injection_exception: Entry::missing(),
            vmm_communication_exception: Entry::missing(),
            security_exception: Entry::missing(),
            reserved3: Entry::missing(),
            interrupts: [Entry::missing(); 256 - 32],
        }
    }

    pub fn load(&'static self) {
        use super::instructions::lidt;

        let pointer = DescriptorTablePointer {
            limit: (size_of::<Self>() - 1) as u16,
            base: self as *const _ as u64,
        };

        unsafe { lidt(&pointer) }
    }
}

impl Default for InterruptDescriptorTable {
    fn default() -> Self {
        Self::new()
    }
}

impl<F> Entry<F> {
    pub const fn missing() -> Self {
        Self {
            ptr_low: 0,
            options: EntryOptions::minimal(),
            ptr_mid: 0,
            ptr_high: 0,
            reserved: 0,
            phantom: PhantomData,
        }
    }

    /// # Safety
    /// Callers must ensure that `handler_addr` is valid.
    pub unsafe fn set_handler_addr(&mut self, handler_addr: u64) {
        self.ptr_low = handler_addr as u16;
        self.ptr_mid = (handler_addr >> 16) as u16;
        self.ptr_high = (handler_addr >> 32) as u32;

        self.options = EntryOptions::minimal();
        self.options.code_selector = CS::read();
        self.options.set_present(true);
    }

    pub fn handler_addr(&self) -> u64 {
        let low = self.ptr_low as u64;
        let mid = (self.ptr_mid as u64) << 16;
        let high = (self.ptr_high as u64) << 32;
        high | mid | low
    }
}

impl<F: InterruptHandlerFn> Entry<F> {
    #[inline]
    pub fn set_handler_fn(&mut self, handler: F) {
        unsafe { self.set_handler_addr(handler.to_addr()) }
    }
}

impl<T> fmt::Debug for Entry<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("handler_addr", &format_args!("{:#x}", self.handler_addr()))
            .field("options", &self.options)
            .finish()
    }
}

impl EntryOptions {
    pub const fn minimal() -> Self {
        Self {
            code_selector: SegmentSelector::NULL,
            bits: 0b0000_1110_0000_0000,
        }
    }

    #[inline]
    pub fn set_present(&mut self, present: bool) {
        self.bits.set_bit(15, present);
    }

    #[inline]
    pub fn present(&self) -> bool {
        self.bits.get_bit(15)
    }

    #[inline]
    pub fn disable_interrupts(&mut self, disable: bool) {
        self.bits.set_bit(8, !disable);
    }

    #[inline]
    pub fn set_privilege_level(&mut self, dpl: u16) {
        self.bits.set_bits(13..15, dpl);
    }

    #[inline]
    pub fn privilege_level(&self) -> u16 {
        self.bits.get_bits(13..15)
    }

    /// ## Safety
    ///
    /// This function is unsafe because the caller must ensure that the passed stack index is
    /// valid and not used by other interrupts. Otherwise, memory safety violations are possible.
    #[inline]
    pub unsafe fn set_stack_index(&mut self, index: u16) {
        self.bits.set_bits(0..3, index);
    }

    #[inline]
    pub fn stack_index(&self) -> u16 {
        self.bits.get_bits(0..3)
    }
}

impl fmt::Debug for EntryOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EntryOptions")
            .field("code_selector", &self.code_selector)
            .field("stack_index", &self.stack_index())
            .field("type", &format_args!("{:#04b}", self.bits.get_bits(8..12)))
            .field("privilege_level", &self.privilege_level())
            .field("present", &self.present())
            .finish()
    }
}

impl fmt::Debug for InterruptStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = f.debug_struct("InterruptStackFrame");
        s.field("instruction_pointer", &self.instruction_pointer);
        s.field("code_segment", &self.code_segment);
        s.field("cpu_flags", &self.cpu_flags);
        s.field("stack_pointer", &self.stack_pointer);
        s.field("stack_segment", &self.stack_segment);
        s.finish()
    }
}

macro_rules! impl_interrupt_handler {
    ($f:ty) => {
        unsafe impl InterruptHandlerFn for $f {
            #[cfg_attr(
                any(target_pointer_width = "32", target_pointer_width = "64"),
                allow(clippy::fn_to_numeric_cast)
            )]
            fn to_addr(self) -> u64 {
                self as u64
            }
        }
    };
}

impl_interrupt_handler!(HandlerFunc);
impl_interrupt_handler!(HandlerFuncWithErr);
impl_interrupt_handler!(DivergingHandlerFunc);
impl_interrupt_handler!(DivergingHandlerFuncWithErr);
