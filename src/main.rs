#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sosp::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use log::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    sosp::init();

    #[cfg(test)]
    test_main();

    info!("Stopped");
    loop {}
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
