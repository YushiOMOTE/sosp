#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sosp::test_runner)]
#![reexport_test_harness_main = "test_main"]

sosp::test_harness!();

#[test_case]
fn test_simple() {
    assert_eq!(1, 1);
}
