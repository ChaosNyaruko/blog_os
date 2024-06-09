#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use blog_os::{exit_qemu, serial_print, serial_println, QemuExitCode};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    blog_os::gdt::init();
    init_test_idt();

    stack_overflow(); // trigger a stack overflow
    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    // 比如我手动让它不爆栈，直接return，这个测试就失败了，因为没有处理到Double Fault里
    // return;
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations -->
                                       // 防止尾递归优化，不然优化成循环的话是不会爆栈的，一会儿可以试一下
                                       // 我们来试一下这个
                                       // 其实也是可以的，看起来编译器没有对这个做尾递归优化😊
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(blog_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

fn init_test_idt() {
    TEST_IDT.load();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]"); // 我们的测试目的就是想让它（指stack_overflow时）走进这里
    exit_qemu(QemuExitCode::Success);
    loop {}
}
