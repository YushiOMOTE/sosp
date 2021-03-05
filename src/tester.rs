use core::panic::PanicInfo;
use log::*;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    com_logger::init();
    crate::test_main();
    loop {}
}

pub fn test_panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic(info);
}

pub fn test_runner(tests: &[&dyn Testable]) {
    info!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        info!("{}: running...", core::any::type_name::<T>());
        self();
        info!("{}: ok", core::any::type_name::<T>());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = x86_64::instructions::port::Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[macro_export]
macro_rules! test_harness {
    () => {
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            com_logger::init();
            test_main();
            loop {}
        }

        #[panic_handler]
        fn panic(info: &core::panic::PanicInfo) -> ! {
            sosp::test_panic(info)
        }
    };
}
