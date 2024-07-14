#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    blog_os::init();

    // insert an int3 to trigger breakpoint exception
    // x86_64::instructions::interrupts::int3();
    //

    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // }

    // This is for "double fault" demonstration
    // fn stack_overflow() {
    //     stack_overflow(); // infinite recursion
    // }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash");

    loop {
        // panic!("Whoops!");
        // use blog_os::print;
        // for _ in 0..10000 {}
        // print!("-");
        blog_os::hlt_loop();
    }
}

#[test_case]
fn xx() {
    assert_eq!(1, 1);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
