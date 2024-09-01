#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{memory::active_level_4_table, println};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::{structures::paging::PageTable, VirtAddr};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello world{}", "!");

    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        // vga buffer identity-mapped -> io
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // real physical 0 start
        boot_info.physical_memory_offset, //-> 0
        // our 0x1000
        0x1000,
        // not mapped, e.g. 0 -> null pointer ---> huge page panic
        0xdeadbeaf, // -> None
        0,          // -> None
    ];

    let mapper = unsafe { memory::init(phys_mem_offset) };
    use blog_os::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
        unsafe {
            let x = *(address as *mut u8);
            println!("{:?} contains {:}", addresses, x);
        };
    }

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
