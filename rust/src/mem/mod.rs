
pub mod heap;
pub mod align;

const  HEAP_KERNEL_SIZE: usize = 5 * 1024;  // 2kB
const  MACHINE_ALIGN: usize = core::mem::size_of::<usize>();
const  HEAP_BLOCK: usize = HEAP_KERNEL_SIZE / MACHINE_ALIGN;
const  HEAP_BLOCK_SIZE: usize = 8 * 1024;
const  HEAP_BLOCK_NUM : usize = HEAP_KERNEL_SIZE/ HEAP_BLOCK_SIZE;

use crate::mem::heap::LockedHeap;

#[no_mangle]
// #[link_section = ".heap._kernel"]
static mut HEAP_KERNEL: [usize; HEAP_BLOCK] = [0; HEAP_BLOCK];

#[global_allocator]
static KERNEL_ALLOCATOR: LockedHeap = LockedHeap::new();

pub unsafe fn heap_init() {
    let bootom = HEAP_KERNEL.as_ptr() as usize;
    let size = HEAP_KERNEL_SIZE;
    KERNEL_ALLOCATOR.init(bootom, size);
}

pub fn allocator() -> &'static LockedHeap {
    &KERNEL_ALLOCATOR
}


