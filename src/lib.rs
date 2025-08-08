#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod memory;
pub mod vga;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, Rust!");

    #[allow(clippy::empty_loop)]
    loop {}
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
