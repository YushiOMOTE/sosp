#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod tester;

pub use crate::tester::{test_panic, test_runner};

pub fn init() {
    com_logger::init();
    log::info!("Starting...");
    crate::gdt::init_gdt();
    crate::interrupts::init_idt();
}
