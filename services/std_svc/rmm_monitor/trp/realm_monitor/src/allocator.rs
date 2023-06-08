use core::mem::MaybeUninit;
use linked_list_allocator::LockedHeap;
use crate::rmm::granule_util::{NR_GRANULES};
//FIXME: Resize the heap
pub const RMM_HEAP_SIZE: usize = 1024*4;

pub unsafe fn init() {
    // crate::print!("allocator init \r\n");
    static mut HEAP: [MaybeUninit<u8>; RMM_HEAP_SIZE] = [MaybeUninit::uninit(); RMM_HEAP_SIZE];
    #[global_allocator]
    static mut ALLOCATOR: LockedHeap = LockedHeap::empty();

    ALLOCATOR.lock().init_from_slice(&mut HEAP);
}
