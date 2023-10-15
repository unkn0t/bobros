#![no_std]
#![no_main]
#![feature(negative_impls)]

mod vga_buffer;
mod locks;

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    loop {}
}
