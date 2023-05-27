use core::{alloc::GlobalAlloc, ptr::NonNull};

use buddy_system_allocator::{Heap, LockedHeap};


#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<64>  = LockedHeap::new();

const MEM_COUNT: usize = 1024 * 1024 * 4;
#[no_mangle]
static MEM: [u8; MEM_COUNT] = [0; MEM_COUNT];


pub fn setup_heap() {
    unsafe {

        HEAP_ALLOCATOR.lock().init(MEM.as_ptr() as usize, MEM_COUNT * core::mem::size_of::<u8>());
    }
}

