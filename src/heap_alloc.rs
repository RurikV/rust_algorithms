use core::ptr;
use std::alloc::{alloc, dealloc, Layout};
use std::marker::Send;
use std::sync::Mutex;

const HEAP_SIZE: usize = 4096;
const ALIGNMENT: usize = 8;

struct Block {
    size: usize,
    next: *mut Block,
}

impl Block {
    const fn new(size: usize) -> Self {
        Block { size, next: ptr::null_mut() }
    }
}

struct BlockPtr(*mut Block);

unsafe impl Send for BlockPtr {}

struct Heap {
    head: BlockPtr,
    memory: [u8; HEAP_SIZE],
}

unsafe impl Sync for Heap {}

impl Heap {
    fn new() -> Self {
        let mut heap = Heap {
            head: BlockPtr(ptr::null_mut()),
            memory: [0; HEAP_SIZE],
        };
        let initial_block = Block::new(HEAP_SIZE);
        let initial_block_ptr = heap.memory.as_mut_ptr() as *mut Block;
        unsafe {
            ptr::write(initial_block_ptr, initial_block);
            heap.head = BlockPtr(initial_block_ptr);
        }
        heap
    }

    fn alloc(&mut self, size: usize) -> *mut u8 {
        let aligned_size = align_size(size);
        let mut current = self.head.0;
        let mut previous: *mut Block = ptr::null_mut();

        while !current.is_null() {
            let block = unsafe { &mut *current };
            if block.size >= aligned_size {
                if block.size == aligned_size {
                    if previous == ptr::null_mut() {
                        self.head = BlockPtr(block.next);
                    } else {
                        unsafe {
                            (*previous).next = block.next;
                        }
                    }
                } else {
                    block.size -= aligned_size;
                    current = unsafe { current.add(block.size) };
                    unsafe {
                        ptr::write(current, Block::new(aligned_size));
                    }
                }
                return current as *mut u8;
            }
            previous = current;
            current = block.next;
        }
        ptr::null_mut()
    }

    fn free(&mut self, ptr: *mut u8) {
        let block_ptr = ptr as *mut Block;
        unsafe {
            (*block_ptr).next = self.head.0;
            self.head = BlockPtr(block_ptr);
        }
    }
}

use once_cell::sync::Lazy;

fn align_size(size: usize) -> usize {
    (size + ALIGNMENT - 1) & !(ALIGNMENT - 1)
}

static ALLOCATOR: Lazy<Mutex<Heap>> = Lazy::new(|| Mutex::new(Heap::new()));

#[no_mangle]
pub extern "C" fn my_malloc(size: usize) -> *mut u8 {
    ALLOCATOR.lock().unwrap().alloc(size)
}

#[no_mangle]
pub extern "C" fn my_free(ptr: *mut u8) {
    ALLOCATOR.lock().unwrap().free(ptr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_alloc_and_free() {
        let mut heap = Heap::new();
        let ptr1 = heap.alloc(8);
        let ptr2 = heap.alloc(16);
        let ptr3 = heap.alloc(32);

        assert!(!ptr1.is_null());
        assert!(!ptr2.is_null());
        assert!(!ptr3.is_null());

        heap.free(ptr2);
        heap.free(ptr1);
        heap.free(ptr3);
    }

    #[test]
    fn test_out_of_memory() {
        let mut heap = Heap::new();
        let mut ptrs = Vec::new();

        while let Some(ptr) = unsafe { heap.alloc(1024).as_mut() } {
            ptrs.push(ptr);
        }

        assert!(ptrs.len() < HEAP_SIZE / 1024);

        for ptr in ptrs {
            heap.free(ptr);
        }
    }

    #[test]
    fn test_performance() {
        const NUM_ITERATIONS: usize = 100_000;

        let start = Instant::now();
        for _ in 0..NUM_ITERATIONS {
            let ptr = my_malloc(8);
            my_free(ptr);
        }
        let elapsed_custom = start.elapsed();

        let start = Instant::now();
        for _ in 0..NUM_ITERATIONS {
            let layout = Layout::new::<u64>();
            let ptr = unsafe { alloc(layout) };
            unsafe { dealloc(ptr, layout) };
        }
        let elapsed_std = start.elapsed();

        println!("Custom allocator: {:?}", elapsed_custom);
        println!("Standard allocator: {:?}", elapsed_std);
    }
}