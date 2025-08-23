pub mod gdt;
pub mod idt;
pub mod instructions;
pub mod registers;
pub mod rflags;
pub mod tss;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed(2))]
pub struct DescriptorTablePointer {
    pub limit: u16,
    pub base: u64,
}
assert_eq_size!(DescriptorTablePointer, [u8; 10]);

impl PrivilegeLevel {
    pub const fn from_u16(value: u16) -> Self {
        match value {
            0 => PrivilegeLevel::Ring0,
            1 => PrivilegeLevel::Ring1,
            2 => PrivilegeLevel::Ring2,
            3 => PrivilegeLevel::Ring3,
            _ => panic!("invalid privilege level"),
        }
    }
}
