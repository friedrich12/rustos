#[cfg(test)]

use crate::{serial_print, QemuExitCode, exit_qemu, serial_println};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;
use crate::gdt;

//static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };  
}

pub fn init_idt(){
    IDT.load();
}

pub fn init(){
    init_idt();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame){
    println!("EXCEPTION BREAKPOINT\n{:#?}", stack_frame);
}

// TODO: check this out
// x86_64 does not permit returning from a double fault
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) /*-> !*/ {
    panic!("EXCPETION DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("testing breakpoint exception...");
    x86_64::instructions::interrupts::int3();
    serial_println!("[passed]");
}
