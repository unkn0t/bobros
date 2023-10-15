#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(bobros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bobros::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    bobros::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    loop {}
}


/// Panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bobros::test_panic_handler(info)
}

