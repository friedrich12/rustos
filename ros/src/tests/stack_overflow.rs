#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use ros::{serial_print, QemuExitCode, exit_qemu, serial_println};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;


// TODO: check this out
// x86_64 does not permit returning from a double fault

extern "x86-interrupt" fn test_double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) /*-> !*/ {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame){
    serial_println!("EXCEPTION BREAKPOINT\n{:#?}", stack_frame);
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(test_double_fault_handler)
                .set_stack_index(ros::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };  
}


pub fn init_test_idt(){
    TEST_IDT.load();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack overflow test...");
    
    ros::gdt::init();
    init_test_idt();

    stack_overflow();
    panic!("Failed to stop execution after stack overflow.");
    loop{}
}

// My test function
#[allow(unconditional_recursion)]
fn stack_overflow() -> !{
    stack_overflow();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}