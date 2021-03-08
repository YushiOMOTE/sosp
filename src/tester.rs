use core::panic::PanicInfo;
use log::*;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_main);

#[cfg(test)]
pub fn test_main(_boot_info: &BootInfo) -> ! {
    crate::init();
    crate::test_main();
    loop {}
}

pub fn test_panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    exit_qemu_failed();
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
    exit_qemu_success();
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

pub fn exit_qemu_success() {
    exit_qemu(QemuExitCode::Success)
}

pub fn exit_qemu_failed() {
    exit_qemu(QemuExitCode::Failed)
}

#[macro_export]
macro_rules! test_harness {
    () => {
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            com_logger::init();
            sosp::interrupts::init_idt();
            test_main();
            loop {}
        }

        #[panic_handler]
        fn panic(info: &core::panic::PanicInfo) -> ! {
            sosp::test_panic(info)
        }
    };
}
