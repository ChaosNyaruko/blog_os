#![no_std]
#![no_main]

use core::panic::PanicInfo;

// TODO:https://github.com/phil-opp/blog_os/issues/1249#issuecomment-2005158679
// Now, if you are on MacOS newer than than version 14, and you are using Qemu, listen up.
// There is a window-sizing bug in Qemu that will shift text up, preventing you from being able to see some of it.
// So I prefix it with a LONGLONGLONGLONG string to let the "Hello World" be seen!
static HELLO: &[u8] =
    b"oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooopppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp\nHello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // VGA text buffer format: 2bytes=ASCII+colour
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
