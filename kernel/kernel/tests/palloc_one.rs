#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use kernel_boot_interface::BootInfo;
use odysseos::memory;

#[no_mangle]
pub extern "C" fn _kernel_start() -> ! {
    test_main();
    kernel_cpu::hcf();
}

#[test_case]
fn palloc(boot_info: &BootInfo) {
    memory::palloc::init(&boot_info.hhdm, &boot_info.memmap);
}

//#[panic_handler]
//pub fn test_panic(info: &core::panic::PanicInfo) -> ! {
//    kernel_test::panic(info);
//}
