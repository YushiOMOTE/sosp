#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sosp::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use log::*;
use x86_64::VirtAddr;

entry_point!(main);

pub fn main(boot_info: &'static BootInfo) -> ! {
    sosp::init();

    #[cfg(test)]
    test_main();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { sosp::memory::init_memory(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { sosp::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    sosp::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    sosp::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sosp::test_panic(info);
}

#[test_case]
fn test1() {
    assert_eq!(1, 1);
}
