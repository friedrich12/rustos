#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![reexport_test_harness_main = "test_main"]

use::core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    test_panic_handler(_info);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Fn()]){
    serial_println!("Running {} tests", tests.len());
    for test in tests{
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    interrupts::init();
    test_main();
    loop {}
}

pub fn init(){
    gdt::init();
    interrupts::init_idt();
    unsafe {interrupts::PICS.lock().initialize()};
    unsafe {asm!("sti")}; // enable interrupts
}