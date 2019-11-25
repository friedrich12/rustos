// Disable the standard library
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_main] // Overwrite the entry point
use::core::panic::PanicInfo;
use::ros::println;

#[cfg(not(test))]
#[panic_handler]
// Tell the compiler we will never return
// a value
fn panic(_info: &PanicInfo) -> !{
    println!("{}",_info);
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    ros::test_panic_handler(_info);
    //loop {}
}

// We will diverage over here because
// _start is invoked by the bootloader
// not called by another functions

// Live forever
static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // The VGA text buffer is located at physical memory address 0xB8000
    //use core::fmt::Write;
    //let w = &vga_buffer::WRITER;
    //w.lock().write_str("HELLO WORLD").unwrap();
    // write!(w.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello World {}", "!");
    // panic!("WFT");
    
    #[cfg(test)]
    test_main();
    loop {}
}
