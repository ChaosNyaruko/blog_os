pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

use core::ptr;

use alloc::alloc::{GlobalAlloc, Layout};

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > bump.heap_end {
            ptr::null_mut() // out of memory, a.k.a. OOM
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut bump = self.lock();

        bump.allocations -= 1;
        // bump.allocation_right_loc
        if bump.allocations == 0 {
            // 有点像引用计数的设计
            bump.next = bump.heap_start;
        }
    }
}

fn _align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        // addr = 4
        // aligh = 4
        // addr 4->4
        addr
    } else {
        // addr = 5
        // aligh = 4
        // 5 % 4 = 1
        // addr 5->8  5 - 1 + 4
        addr - remainder + align
    }
}

/// A more effifient way of aligning up, but requires `align` is a power of two
fn align_up(addr: usize, align: usize) -> usize {
    // align: 1 << i -> 000000010000000000
    // align: 1 << i -> 111111110000000000
    // -1
    // 1000 -> 0111
    // (addr + align - 1) & align
    (addr + align - 1) & !(align - 1)
    // addr            :101101100110101010
    // addr            :101101110000000000
}
