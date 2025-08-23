use bit_field::BitField;
use bitflags::bitflags;

use crate::x86_64::registers::SegmentSelector;
use crate::x86_64::tss::TaskStateSegment;
use crate::x86_64::{DescriptorTablePointer, PrivilegeLevel};

#[derive(Clone, Debug)]
pub struct GlobalDescriptorTable<const MAX: usize = 8> {
    entries: [u64; MAX],
    len: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Descriptor {
    UserSegment(u64),
    SystemSegment { low: u64, high: u64 },
}

bitflags! {
    struct DescriptorFlags: u64 {
        const ACCESSED          = 1 << 40;
        const WRITABLE          = 1 << 41;
        const CONFORMING        = 1 << 42;
        const EXECUTABLE        = 1 << 43;
        const USER_SEGMENT      = 1 << 44;
        const DPL_RING_3        = 3 << 45;
        const PRESENT           = 1 << 47;
        const AVAILABLE         = 1 << 52;
        const LONG_MODE         = 1 << 53;
        const DEFAULT_SIZE      = 1 << 54;
        const GRANULARITY       = 1 << 55;
    }
}

impl GlobalDescriptorTable {
    pub const fn new() -> Self {
        Self::empty()
    }
}

impl Default for GlobalDescriptorTable {
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX: usize> GlobalDescriptorTable<MAX> {
    pub const fn empty() -> Self {
        const {
            assert!(MAX > 0, "GDT has at least one entry");
            assert!(MAX < 8192, "GDT can at most have 2^13 entries");
        }

        Self {
            entries: [0; MAX],
            len: 1,
        }
    }

    pub const fn add_entry(&mut self, entry: Descriptor) -> SegmentSelector {
        let index = match entry {
            Descriptor::UserSegment(value) => self.push(value),
            Descriptor::SystemSegment { low, high } => {
                let index = self.push(low);
                self.push(high);
                index
            }
        };

        SegmentSelector::new(index as u16, PrivilegeLevel::Ring0)
    }

    pub fn load(&'static self) {
        use super::instructions::lgdt;

        let pointer = DescriptorTablePointer {
            limit: (self.len * size_of::<u64>() - 1) as u16,
            base: self.entries.as_ptr() as u64,
        };

        unsafe { lgdt(&pointer) }
    }

    const fn push(&mut self, value: u64) -> usize {
        if self.len < self.entries.len() {
            let index = self.len;
            self.entries[index] = value;
            self.len += 1;
            index
        } else {
            panic!("GDT full");
        }
    }
}

impl DescriptorFlags {
    pub const COMMON: Self = Self::USER_SEGMENT
        .union(Self::PRESENT)
        .union(Self::WRITABLE)
        .union(Self::ACCESSED)
        .union(Self::GRANULARITY);

    pub const KERNEL_CODE64: Self = Self::COMMON.union(Self::EXECUTABLE).union(Self::LONG_MODE);
}

impl Descriptor {
    pub const fn kernel_code_segment() -> Self {
        Self::UserSegment(DescriptorFlags::KERNEL_CODE64.bits())
    }

    pub fn tss_segment(tss: &'static TaskStateSegment) -> Self {
        let ptr = tss as *const _ as u64;

        let mut low = DescriptorFlags::PRESENT.bits();
        // base
        low.set_bits(16..40, ptr.get_bits(0..24));
        low.set_bits(56..64, ptr.get_bits(24..32));
        // limit
        low.set_bits(0..16, (size_of::<TaskStateSegment>() - 1) as u64);
        // type
        low.set_bits(40..44, 0b1001);

        let mut high = 0;
        high.set_bits(0..32, ptr.get_bits(32..64));

        Descriptor::SystemSegment { low, high }
    }
}
