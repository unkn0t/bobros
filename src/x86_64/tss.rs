#[repr(C, packed(4))]
pub struct TaskStateSegment {
    reserved1: u32,
    pub privilege_stack_table: [u64; 3],
    reserved2: u64,
    pub interrupt_stack_table: [u64; 7],
    reserved3: u64,
    reserved4: u16,
    pub io_map_base_address: u16,
}
assert_eq_size!(TaskStateSegment, [u8; 0x68]);

impl TaskStateSegment {
    pub const fn new() -> Self {
        Self {
            privilege_stack_table: [0; 3],
            interrupt_stack_table: [0; 7],
            io_map_base_address: size_of::<Self>() as u16,
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
            reserved4: 0,
        }
    }
}

impl Default for TaskStateSegment {
    fn default() -> Self {
        Self::new()
    }
}
