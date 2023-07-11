use kernel_cpu;
use kernel_log::kprintln;

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    kprintln!("{:?}", info);
    kernel_cpu::hcf();
}

#[cfg(test)]
#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    kernel_test::panic(info);
}
