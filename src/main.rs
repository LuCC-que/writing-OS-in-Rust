#![no_std]
#![no_main]
//the CTF gens a test_runner as alias of main function
//however, as no_main, this will be ignore
//thus, we rename our main as test_main
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{init, println};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    init(); //init the IDT for the CPU

    // //invoke any exception
    // //if it doest know the handler, qemu will re-boot the machine all the time
    // unsafe { *(0xdeadbeef as *mut u64) = 64 } //deadbeef cant turn to valid pa, case page fault

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
