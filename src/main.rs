#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use blog_os::memory;
use blog_os::{allocator, memory::active_level_4_table, println};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::{
    structures::paging::{Page, PageTable},
    VirtAddr,
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello world{}", "!");

    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    use blog_os::memory::BootInfoFrameAllocator;
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // let addresses = [
    //     // vga buffer identity-mapped -> io
    //     0xb8000,
    //     // some code page
    //     // 0x201008,
    //     // some stack page
    //     // 0x0100_0020_1a10,
    //     // real physical 0 start
    //     // boot_info.physical_memory_offset, //-> 0
    //     0x0,
    //     0xdeadbeaf000,
    // ];

    // use x86_64::{structures::paging::Translate, VirtAddr};
    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();

    // // literal "New!"
    // // x86 is little-endian
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    allocator::init_heap(&mut mapper, &mut frame_allocator);
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let r_count = Rc::new(vec![1, 2, 3]);
    let cloned = r_count.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned));
    core::mem::drop(r_count);
    println!("reference count is {} now", Rc::strong_count(&cloned));
    let y = &cloned[1];
    println!("{y}");

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
