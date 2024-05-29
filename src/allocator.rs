use core::ptr;
use core::mem;

pub struct Allocator<'mem> {
    remaining_memory: &'mem mut [u8],
}

#[rustfmt::skip]
impl<'mem> Allocator<'mem> {
    pub fn new(heap: &'mem mut [u8]) -> Self {
        Allocator { remaining_memory: heap }
    }


    pub fn free<T>(&mut self, item: &'mem mut T) {
        unsafe {
            let ptr = item as *mut T as *mut u8;
            let size = mem::size_of::<T>();
            let remaining_mem = mem::take(&mut self.remaining_memory);
            let (freed_mem, remaining_mem) = remaining_mem.split_at_mut(size);
            ptr::copy_nonoverlapping(ptr, freed_mem.as_mut_ptr(), size);
            self.remaining_memory = remaining_mem;
        }
    }

    pub fn allocate<'item, T>(&mut self, item: T) -> AllocResult<&'item mut T>
    where
        'mem: 'item
    {
        self.align_memory::<T>()?;
        unsafe { self.allocate_aligned(item) }
    }

    fn align_memory<T>(&mut self) -> AllocResult<()> {
        let align = mem::align_of::<T>();
        let bytes_to_skip: usize = {
            let memory_ptr = self.remaining_memory.as_ptr() as usize;
            let mut temp = memory_ptr;
            while temp % align != 0 {
                temp += 1; // There is a more efficient way
            }
            temp - memory_ptr
        };

        if self.remaining_memory.len() < bytes_to_skip {
            return Err(OutOfMemory);
        }

        let remaining_mem = mem::take(&mut self.remaining_memory);
        let (_, remaining_mem) = remaining_mem.split_at_mut(bytes_to_skip);
        self.remaining_memory = remaining_mem;

        Ok(())
    }

    // SAFETY: This function should only be called after
    // self.remaining_memory has been aligned for T.
    unsafe fn allocate_aligned<'item, T>(&mut self, item: T) -> AllocResult<&'item mut T>
    where
        'mem: 'item
    {
        let size = mem::size_of::<T>();

        if self.remaining_memory.len() < size {
            return Err(OutOfMemory);
        }

        let remaining_mem = mem::take(&mut self.remaining_memory);
        let (item_memory, remaining_mem) = remaining_mem.split_at_mut(size);
        self.remaining_memory = remaining_mem;

        let item_ref: &mut T = {
            let item_ptr = item_memory as *mut [u8] as *mut T;
            unsafe {
                core::ptr::write(item_ptr, item);
                &mut *item_ptr
            }
        };
        Ok(item_ref)
    }

    pub fn allocate_array<'arr, T>(
        &mut self,
        size: usize,
        mut init_item: impl FnMut(usize) -> T,
    ) -> AllocResult<&'arr mut [T]>
    where
        'mem: 'arr,
    {
        self.align_memory::<T>()?;

        if self.remaining_memory.len() < size {
            return Err(OutOfMemory);
        }

        let array_ptr = self.remaining_memory as *mut [u8] as *mut [T];

        for i in 0..size {
            let _ = unsafe { self.allocate_aligned(init_item(i)).unwrap_unchecked() };
        }

        Ok(&mut unsafe { &mut *array_ptr }[0..size])
    }
}

pub type AllocResult<T> = core::result::Result<T, OutOfMemory>;

#[derive(Debug)]
pub struct OutOfMemory;

#[rustfmt::skip]
pub fn count_primes(nums: &[u32], max_num: u32, allocator: &mut Allocator) -> usize {
    match FastPrimeTable::allocate_in(max_num, allocator) {
        Ok(table) => return count_primes_with_table(nums, table),
        Err(OutOfMemory) => {}
    };

    match SmallPrimeTable::allocate_in(max_num, allocator) {
        Ok(table) => return count_primes_with_table(nums, table),
        Err(OutOfMemory) => {}
    };

    count_primes_with_table(nums, RawPrimesTable)
}

fn count_primes_with_table(nums: &[u32], table: impl PrimeTable) -> usize {
    nums.iter().filter(|&&num| table.is_prime(num)).count()
}

trait PrimeTable {
    fn is_prime(&self, n: u32) -> bool;
}

struct SmallPrimeTable<'tab> {
    raw: &'tab [u32],
}

impl<'tab> SmallPrimeTable<'tab> {
    fn allocate_in<'mem>(max_num: u32, allocator: &mut Allocator<'mem>) -> AllocResult<Self>
    where
        'mem: 'tab,
    {
        let n_primes = count_primes_until(max_num);
        let mut primes = RawPrimesIter::new();
        let prime_table = allocator.allocate_array::<u32>(n_primes, |_| primes.next().unwrap())?;

        Ok(Self { raw: prime_table })
    }
}

fn count_primes_until(max_num: u32) -> usize {
    RawPrimesIter::new()
        .take_while(|&prime| prime <= max_num)
        .count()
}

impl<'tab> PrimeTable for SmallPrimeTable<'tab> {
    fn is_prime(&self, n: u32) -> bool {
        self.raw.contains(&n)
    }
}

struct FastPrimeTable<'tab> {
    raw: &'tab [Primality],
}

enum Primality {
    Prime,
    Composite,
}

impl<'tab> FastPrimeTable<'tab> {
    fn allocate_in<'mem>(max_num: u32, allocator: &mut Allocator<'mem>) -> AllocResult<Self>
    where
        'mem: 'tab,
    {
        let prime_table = allocator.allocate_array::<Primality>(
            max_num as usize + 1,
            |_| Primality::Composite,
        )?;
        for num in 0..=max_num {
            if is_prime_raw(num) {
                prime_table[num as usize] = Primality::Prime;
            }
        }
        Ok(Self { raw: prime_table })
    }
}

impl<'tab> PrimeTable for FastPrimeTable<'tab> {
    fn is_prime(&self, n: u32) -> bool {
        match self.raw[n as usize] {
            Primality::Prime => true,
            Primality::Composite => false,
        }
    }
}

struct RawPrimesTable;

impl PrimeTable for RawPrimesTable {
    fn is_prime(&self, n: u32) -> bool {
        is_prime_raw(n)
    }
}

struct RawPrimesIter(u32);

impl RawPrimesIter {
    fn new() -> Self {
        Self(1)
    }
}

impl Iterator for RawPrimesIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.0 += 1;

            if is_prime_raw(self.0) {
                return Some(self.0);
            }
        }
    }
}

#[rustfmt::skip]
fn is_prime_raw(num: u32) -> bool {
    if num == 0 || num == 1 { return false; }

    let mut is_prime_flag = true;
    
    for n in 2..=(num / 2) {
        if num % n == 0 {
            is_prime_flag = false;
            break;
        }
    }
    is_prime_flag
}

use std::alloc::{alloc, dealloc, Layout};
use std::time::Instant;

const NUM_ITERATIONS: usize = 100_000;

fn bench_custom_allocator(heap_size: usize) {
    let mut heap: Vec<u8> = vec![0; heap_size];
    let mut allocator = Allocator::new(&mut heap);

    let start = Instant::now();
    let mut success_count = 0;
    for _ in 0..NUM_ITERATIONS {
        let val = 42;
        match allocator.allocate(val) {
            Ok(allocated) => {
                success_count += 1;
                allocator.free(allocated);
            }
            Err(OutOfMemory) => break,
        }
    }
    let elapsed = start.elapsed();
    println!(
        "Custom Allocator - Heap Size: {}, Successful Allocations: {}, Time: {:?}",
        heap_size, success_count, elapsed
    );
}

fn bench_std_allocator() {
    let start = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        let val = 42;
        let layout = Layout::new::<i32>();
        let ptr = unsafe { alloc(layout) as *mut i32 };
        unsafe {
            std::ptr::write(ptr, val);
            dealloc(ptr as *mut u8, layout);
        }
    }
    let elapsed = start.elapsed();
    println!("Standard Allocator - Time: {:?}", elapsed);
}


#[rustfmt::skip]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works_with_bytes() {
        let mut pseudo_heap: [u8; 4] = core::array::from_fn(|_| 9);
        let mut allocator = Allocator::new(&mut pseudo_heap);

        allocator.allocate::<u8>(1).unwrap();
        allocator.allocate::<u8>(2).unwrap();

        assert_eq!(pseudo_heap, [1, 2, 9, 9]);
    }

    #[test]
    fn works_with_i64() {
        let mut pseudo_heap: [u8; 32] = core::array::from_fn(|_| 9);
        let mut allocator = Allocator::new(&mut pseudo_heap);

        let _ = allocator.allocate::<i64>(1).unwrap();
        let _ = allocator.allocate::<i64>(2).unwrap();

        assert_eq!(
            pseudo_heap,
            [
                1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                9, 9, 9, 9,
            ]
        );
    }

    #[test]
    fn works_with_alignment() {
        let mut pseudo_heap: [u8; 16] = core::array::from_fn(|_| 9);
        let mut allocator = Allocator::new(&mut pseudo_heap);

        let u8_addr = allocator.allocate::<u8>(1).unwrap();

        assert!(u8_addr as *mut u8 as usize % 2 == 0);

        let _ = allocator.allocate::<u16>(2).unwrap();

        assert_eq!(pseudo_heap, [1, 9, 2, 0, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
    }

    #[test]
    fn works_with_arrays() {
        let mut pseudo_heap: [u8; 16] = core::array::from_fn(|_| 9);
        let mut allocator = Allocator::new(&mut pseudo_heap);

        let _ = allocator.allocate::<u8>(1).unwrap();
        let _ = allocator.allocate_array::<u16>(3, |_| 2).unwrap();

        assert_eq!(pseudo_heap, [1, 9, 2, 0, 2, 0, 2, 0, 9, 9, 9, 9, 9, 9, 9, 9]);
    }

    #[test]
    fn is_prime_raw_test() {
        assert!(!is_prime_raw(0));
        assert!(!is_prime_raw(1));
        assert!(is_prime_raw(2));
        assert!(is_prime_raw(3));
        assert!(!is_prime_raw(4));
        assert!(is_prime_raw(5));
        assert!(!is_prime_raw(6));
        assert!(is_prime_raw(7));
        assert!(!is_prime_raw(8));
        assert!(!is_prime_raw(9));
        assert!(!is_prime_raw(10));
        assert!(is_prime_raw(11));
        assert!(!is_prime_raw(12));
    }

    #[test]
    fn count_primes_until_test() {
        assert_eq!(count_primes_until(2), 1);
        assert_eq!(count_primes_until(3), 2);
        assert_eq!(count_primes_until(11), 5);
        assert_eq!(count_primes_until(12), 5);
    }

    #[test]
    fn raw_primes_iter_test() {
        let mut primes = RawPrimesIter::new();

        assert_eq!(primes.next(), Some(2));
        assert_eq!(primes.next(), Some(3));
        assert_eq!(primes.next(), Some(5));
        assert_eq!(primes.next(), Some(7));
        assert_eq!(primes.next(), Some(11));
        assert_eq!(primes.next(), Some(13));
        assert_eq!(primes.next(), Some(17));
        assert_eq!(primes.next(), Some(19));
        assert_eq!(primes.next(), Some(23));
    }

    #[test]
    fn raw_primes_table_test() {
        let table = RawPrimesTable;

        assert!(!table.is_prime(0));
        assert!(!table.is_prime(1));
        assert!(table.is_prime(2));
        assert!(table.is_prime(3));
        assert!(!table.is_prime(4));
        assert!(table.is_prime(5));
        assert!(!table.is_prime(6));
        assert!(table.is_prime(7));
        assert!(!table.is_prime(8));
        assert!(!table.is_prime(9));
        assert!(!table.is_prime(10));
        assert!(table.is_prime(11));
        assert!(!table.is_prime(12));
        assert!(table.is_prime(13));
    }

    #[test]
    fn small_primes_table_test() {
        let mut heap: [u8; 1024] = core::array::from_fn(|_| 0);
        let mut allocator = Allocator::new(&mut heap);

        let table = SmallPrimeTable::allocate_in(50, &mut allocator).unwrap();

        assert!(!table.is_prime(0));
        assert!(!table.is_prime(1));
        assert!(table.is_prime(2));
        assert!(table.is_prime(3));
        assert!(!table.is_prime(4));
        assert!(table.is_prime(5));
        assert!(!table.is_prime(6));
        assert!(table.is_prime(7));
        assert!(!table.is_prime(8));
        assert!(!table.is_prime(9));
        assert!(!table.is_prime(10));
        assert!(table.is_prime(11));
        assert!(!table.is_prime(12));
        assert!(table.is_prime(13));
    }

    #[test]
    fn fast_primes_table_test() {
        let mut heap: [u8; 2048] = core::array::from_fn(|_| 0);
        let mut allocator = Allocator::new(&mut heap);

        let table = FastPrimeTable::allocate_in(50, &mut allocator).unwrap();

        assert!(!table.is_prime(0));
        assert!(!table.is_prime(1));
        assert!(table.is_prime(2));
        assert!(table.is_prime(3));
        assert!(!table.is_prime(4));
        assert!(table.is_prime(5));
        assert!(!table.is_prime(6));
        assert!(table.is_prime(7));
        assert!(!table.is_prime(8));
        assert!(!table.is_prime(9));
        assert!(!table.is_prime(10));
        assert!(table.is_prime(11));
        assert!(!table.is_prime(12));
        assert!(table.is_prime(13));
    }

    #[test]
    fn count_primes_test() {
        let mut heap: [u8; 1024] = core::array::from_fn(|_| 0);
        let mut allocator = Allocator::new(&mut heap);

        let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let n_primes = count_primes(&nums, 11, &mut allocator);

        assert_eq!(n_primes, 5);
    }

    #[test]
    fn count_primes_multiple_test() {
        let mut heap: [u8; 1024] = core::array::from_fn(|_| 0);
        let mut allocator = Allocator::new(&mut heap);

        let nums = [1, 2, 2, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11];
        let n_primes = count_primes(&nums, 11, &mut allocator);

        assert_eq!(n_primes, 7);
    }
}

fn main() {
    print_allocated_after_alloc_was_dropped();
    visualize_layout();

    bench_custom_allocator(1024);
    bench_custom_allocator(2048);
    bench_custom_allocator(4096);
    bench_std_allocator();
}

pub fn visualize_layout() {
    let mut heap: [u8; 64] = std::array::from_fn(|_| 9);
    let mut allocator = Allocator::new(&mut heap);

    let _: &u8 = allocator.allocate(8).unwrap();
    let _: &u64 = allocator.allocate(64).unwrap();
    let _: &u16 = allocator.allocate(16).unwrap();
    let _: &u32 = allocator.allocate(32).unwrap();

    println!("heap: {:?}", heap);
}

fn print_allocated_after_alloc_was_dropped() {
    let mut heap: [u8; 32] = Default::default();
    let mut allocator = Allocator::new(&mut heap);

    let allocated_num = allocator.allocate::<u32>(873).unwrap();

    drop(allocator);

    println!("num: {allocated_num}");
}