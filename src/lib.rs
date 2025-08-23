#![no_std]
#![feature(abi_x86_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod gdt;
pub mod interrupts;
#[macro_use]
pub mod macros;
pub mod memory;
#[macro_use]
pub mod vga;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, BobrOS!");

    init();

    crate::x86_64::instructions::int3();

    println!("Did not crash!");

    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO: print ColorCode to change color
    vga::WRITER
        .lock()
        .set_color(vga::ColorCode::new(vga::Color::Red, vga::Color::Black));
    println!("{info}");
    loop {}
}
