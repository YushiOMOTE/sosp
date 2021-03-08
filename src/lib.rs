#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod tester;
pub use crate::tester::{test_panic, test_runner};
pub mod allocator;
use log::*;

pub fn init() {
    com_logger::init();
    info!("Starting...");
    crate::gdt::init_gdt();
    crate::interrupts::init_idt();
}

pub fn hlt_loop() -> ! {
    info!("Stopped");
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
