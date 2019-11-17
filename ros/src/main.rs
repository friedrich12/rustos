// Disable the standard library
#![no_std]
#![no_main] // Overwrite the entry point
use::core::panic::PanicInfo;

mod vga_buf;

#[panic_handler]
// Tell the compiler we will never return
// a value
fn panic(_info: &PanicInfo) -> !{
    loop {}
}


// We will diverage over here because
// _start is invoked by the bootloader
// not called by another functions

// Live forever
static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // The VGA text buffer is located at physical memory address 0xB8000
    let vga_buf =  0xb8000 as *mut u8;
    let mut a = 0x1;
    for(i, &byte) in HELLO.iter().enumerate(){
        unsafe{
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0x1 + a; // Colors
            a += 0x1;
        }
    }

    loop {}
}