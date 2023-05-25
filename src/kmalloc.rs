use core::{alloc::GlobalAlloc, ptr::NonNull};

use buddy_system_allocator::{Heap};

pub struct FakeMutex<T>(T);

unsafe impl<T> Sync for FakeMutex<T> {}
unsafe impl<T> Send for FakeMutex<T> {}

pub struct SingleCoreHeap<const ORDER: usize>(FakeMutex<Heap<ORDER>>);

#[global_allocator]
static HEAP_ALLOCATOR: SingleCoreHeap<64>  = SingleCoreHeap(FakeMutex(Heap::<64>::empty()));


unsafe impl<const ORDER: usize> Sync for SingleCoreHeap<ORDER> {}
unsafe impl<const ORDER: usize> Send for SingleCoreHeap<ORDER> {}

unsafe impl<const ORDER: usize> GlobalAlloc for SingleCoreHeap<ORDER> {
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        
    }

     unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
         todo!()
    }

}
