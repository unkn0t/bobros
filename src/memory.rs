use core::ptr;

#[repr(transparent)]
pub struct Volatile<T>(T);

impl<T: Clone + Copy> Volatile<T> {
    pub fn read(&self) -> T {
        // SAFETY: Underline type is properly initialized and copyable
        unsafe { ptr::read_volatile(&raw const self.0) }
    }

    pub fn write(&mut self, value: T) {
        // SAFETY: Underline type is properly initialized and copyable
        unsafe { ptr::write_volatile(&raw mut self.0, value) }
    }
}
