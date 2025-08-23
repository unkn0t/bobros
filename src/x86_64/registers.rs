use core::arch::asm;
use core::fmt;

use crate::x86_64::PrivilegeLevel;

pub trait SegmentReg {
    fn read() -> SegmentSelector;

    /// # Safety
    /// Caller must ensure that `selector` is valid segment selector value
    unsafe fn write(selector: SegmentSelector);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    pub const NULL: Self = Self::new(0, PrivilegeLevel::Ring0);

    #[inline]
    pub const fn new(index: u16, rpl: PrivilegeLevel) -> Self {
        Self(index << 3 | (rpl as u16))
    }

    #[inline]
    pub const fn index(self) -> u16 {
        self.0 >> 3
    }

    #[inline]
    pub const fn rpl(self) -> PrivilegeLevel {
        PrivilegeLevel::from_u16(self.0 & 3)
    }
}

impl fmt::Debug for SegmentSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("SegmentSelector");
        s.field("index", &self.index());
        s.field("rpl", &self.rpl());
        s.finish()
    }
}

#[derive(Debug)]
pub struct CS;

impl SegmentReg for CS {
    #[inline]
    fn read() -> SegmentSelector {
        let segment: u16;
        unsafe {
            asm!("mov {0:x}, cs", out(reg) segment, options(nomem, nostack, preserves_flags));
        }
        SegmentSelector(segment)
    }

    /// Note this is special since we cannot directly move to [`CS`]; x86 requires the instruction
    /// pointer and [`CS`] to be set at the same time. To do this, we push the new segment selector
    /// and return value onto the stack and use a "far return" (`retfq`) to reload [`CS`] and
    /// continue at the end of our function.
    ///
    /// Note we cannot use a "far call" (`lcall`) or "far jmp" (`ljmp`) to do this because then we
    /// would only be able to jump to 32-bit instruction pointers. Only Intel implements support
    /// for 64-bit far calls/jumps in long-mode, AMD does not.
    #[inline]
    unsafe fn write(selector: SegmentSelector) {
        unsafe {
            asm!(
                "push {selector}",
                "lea {tmp}, [55f + rip]",
                "push {tmp}",
                "retfq",
                "55:",
                selector = in(reg) selector.0 as u64,
                tmp = lateout(reg) _,
                options(preserves_flags)
            );
        }
    }
}
