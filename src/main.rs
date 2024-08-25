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

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    // let ptr = 0xdeadbeef as *mut u8;
    // unsafe { *ptr = 42 }
    // TODO: what is the content?
    let ptr = 0x1000 as *mut u64;
    let x = unsafe { *ptr };
    println!("read worked: 0x{:X}", x);
    // write should be not ok
    // unsafe { *ptr = 42 }
    // println!("write worked");

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

    blog_os::hlt_loop();
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
