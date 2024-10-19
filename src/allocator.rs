pub mod bump;
pub mod fixed_size_block;
pub mod linked_list;

use alloc::alloc::{GlobalAlloc, Layout};
use bump::BumpAllocator;
use bump::Locked;
use core::ptr::null_mut;
use fixed_size_block::FixedSizeBlockAllocator;
use linked_list_allocator::LockedHeap;

#[global_allocator]
// static ALLOCATOR: LockedHeap = LockedHeap::empty();
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
// static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());
// static ALLOCATOR: Dummy = Dummy;

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        panic!("dealloc should never be called")
    }
}

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; //100 KiB

use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64; // [start, end]
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
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
