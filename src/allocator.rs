
use core::mem;

pub struct Alloc<'mem> {
    remaining_mem: &'mem mut [u8],
}

#[rustfmt::skip]
impl<'mem> Alloc<'mem> {

    pub fn new(heap: &'mem mut [u8]) -> Self {
        Alloc { remaining_mem: heap }
    }

    pub fn alloc<'item, T>(&mut self, item: T) -> 
     AllocResult<&'item mut T>
    where
        'mem: 'item
    {
        self.waste_some_mem_to_reach_align::<T>()?;

        unsafe { self.alloc_aligned(item) }
    }

    fn waste_some_mem_to_reach_align<T>(&mut self) -> AllocResult<()> {

        let align = mem::align_of::<T>();

        let how_many_bytes_to_waste: usize = {

            let remainig_memory_ptr = self.remaining_mem.as_ptr() as usize;

            let mut temp = remainig_memory_ptr;

            while temp % align != 0 {
                temp += 1; // There is more efficient way
            }
            temp - remainig_memory_ptr
        };

        if self.remaining_mem.len() < how_many_bytes_to_waste {
            return Err(OutOfMemory);
        }

        let remaining_memory = mem::take(&mut self.remaining_mem);
        let (_, remaining_memory) = remaining_memory.split_at_mut(how_many_bytes_to_waste);
        self.remaining_mem = remaining_memory;

        Ok(())
    }

    // SAFETY: This function has to be called only after
    // self.remaining_memory was aligned for T.
    unsafe fn alloc_aligned<'item, T>(&mut self, item: T) -> AllocResult<&'item mut T>
    where
        'mem: 'item
    {
        let size = mem::size_of::<T>();

        if self.remaining_mem.len() < size {
            return Err(OutOfMemory);
        }

        let remaining_mem = mem::take(&mut self.remaining_mem);

        let (almost_item_ref, remaining_memory) = remaining_mem.split_at_mut(size);

        self.remaining_mem = remaining_memory;

        let item_ref: &mut T = {
            let almost_item_ptr = almost_item_ref as *mut [u8] as *mut T;

            unsafe {
                core::ptr::write(almost_item_ptr, item);
                &mut *almost_item_ptr // *mut T as &mut T
            }
        };
        Ok(item_ref)
    }

    pub fn alloc_array_from_fn<'arr, T>(
        &mut self, size: usize, mut init_t: impl FnMut(usize) -> T
    ) 
            -> AllocResult<&'arr mut [T]>
    where 'mem: 'arr {

        self.waste_some_mem_to_reach_align::<T>()?;

        if self.remaining_mem.len() < size {
            return Err(OutOfMemory);
        }

        let arr_ptr = self.remaining_mem as *mut [u8] as *mut [T];

        for i in 0..size {
            let _ = unsafe { 
                self.alloc_aligned(init_t(i)).unwrap_unchecked() 
            };
        };

        Ok(&mut unsafe { &mut *arr_ptr }[0..size])
    }
}

pub type AllocResult<T> = core::result::Result<T, OutOfMemory>;

#[derive(Debug)]
pub struct OutOfMemory;

#[rustfmt::skip]
pub fn count_primes(nums: &[u32], max_num: u32, alloc: &mut Alloc) -> usize {
    
    match FastPrimeTable::alloc_in(max_num, alloc) {
        Ok(table) => return count_primes_with_table(nums, table),
        Err(OutOfMemory) => {}
    };

    match SmallPrimeTable::alloc_in(max_num, alloc) {
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
    fn alloc_in<'mem>(max_num: u32, alloc: &mut Alloc<'mem>) -> AllocResult<Self>
    where
        'mem: 'tab,
    {
        let n_primes = count_primes_until(max_num);

        let mut primes = RawPrimesIter::new();

        let almost_prime_table =
            alloc.alloc_array_from_fn::<u32>(n_primes, |_| primes.next().unwrap())?;

        Ok(Self {
            raw: almost_prime_table,
        })
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

enum Primality { Prime, Composite }

impl<'tab> FastPrimeTable<'tab> {
    fn alloc_in<'mem>(max_num: u32, alloc: &mut Alloc<'mem>) -> AllocResult<Self>
    where
        'mem: 'tab,
    {
        let almost_prime_table = alloc.alloc_array_from_fn::<Primality>(
            max_num as usize + 1, |_| Primality::Composite
        )?;
        for num in 0..=max_num {
            if is_prime_raw(num) {
                almost_prime_table[num as usize] = Primality::Prime;
            }
        }
        Ok(Self { raw: almost_prime_table })
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

#[rustfmt::skip]
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn works_with_bytes() {
        let mut pseudo_heap: [u8; 4] = core::array::from_fn(|_| 9);

        let mut alloc = Alloc::new(&mut pseudo_heap);

        alloc.alloc::<u8>(1).unwrap();
        alloc.alloc::<u8>(2).unwrap();

        assert_eq!(pseudo_heap, [1, 2, 9, 9])
    }

    #[test]
    fn works_with_i64() {
        let mut pseudo_heap: [u8; 32] = core::array::from_fn(|_| 9);

        let mut alloc = Alloc::new(&mut pseudo_heap);

        let _ = alloc.alloc::<i64>(1).unwrap();
        let _ = alloc.alloc::<i64>(2).unwrap();

        assert_eq!(pseudo_heap, 
            [
                1, 0, 0, 0, 0, 0, 0, 0,
                2, 0, 0, 0, 0, 0, 0, 0,
                9, 9, 9, 9, 9, 9, 9, 9,
                9, 9, 9, 9, 9, 9, 9, 9,
            ]
        );
    }

    #[test]
    fn works_with_alignment() {

        let mut pseudo_heap: [u8; 16] = core::array::from_fn(|_| 9);

        let mut alloc = Alloc::new(&mut pseudo_heap);

        let u8_addr = alloc.alloc::<u8>(1).unwrap();

        assert!(u8_addr as *mut u8 as usize % 2 == 0);

        let _ = alloc.alloc::<u16>(2).unwrap();

        assert_eq!(pseudo_heap,
            [
                1, 9, 2, 0,
                9, 9, 9, 9,
                9, 9, 9, 9,
                9, 9, 9, 9,
            ]
        );
    }

    #[test]
    fn works_with_arrays() {

        let mut pseudo_heap: [u8; 16] = core::array::from_fn(|_| 9);

        let mut alloc = Alloc::new(&mut pseudo_heap);

        let _ = alloc.alloc::<u8>(1).unwrap();

        let _  = alloc.alloc_array_from_fn::<u16>(3, |_| 2).unwrap();

        assert_eq!(pseudo_heap,
            [
                1, 9, 2, 0,
                2, 0, 2, 0,
                9, 9, 9, 9,
                9, 9, 9, 9,
            ]
        );
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
        let mut alloc = Alloc::new(&mut heap);

        let table = SmallPrimeTable::alloc_in(50, &mut alloc).unwrap();

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
        let mut alloc = Alloc::new(&mut heap);

        let table = FastPrimeTable::alloc_in(50, &mut alloc).unwrap();

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

        let mut alloc = Alloc::new(&mut heap);

        let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

        let n_primes = count_primes(&nums, 11, &mut alloc);

        assert_eq!(n_primes, 5);
    }

    #[test]
    fn count_primes_multiple_test() {
        let mut heap: [u8; 1024] = core::array::from_fn(|_| 0);

        let mut alloc = Alloc::new(&mut heap);

        let nums = [1, 2, 2, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11];

        let n_primes = count_primes(&nums, 11, &mut alloc);

        assert_eq!(n_primes, 7);
    }

    #[test]
    fn bench_ways() {
        
    }
}

fn main() {
    print_allocated_after_alloc_was_dropped();
    visualize_layout();
}

pub fn visualize_layout() {
    let mut heap: [u8; 64] = std::array::from_fn(|_| 9);

    let mut alloc = Alloc::new(&mut heap);

    let _: &u8 = alloc.alloc(8).unwrap();
    let _: &u64 = alloc.alloc(64).unwrap();
    let _: &u16 = alloc.alloc(16).unwrap();
    let _: &u32 = alloc.alloc(32).unwrap();

    println!("heap: {:?}", heap);
}

fn print_allocated_after_alloc_was_dropped() {
    let mut heap: [u8; 32] = Default::default();
    let mut alloc = Alloc::new(&mut heap);

    let allocated_num = alloc.alloc::<u32>(873).unwrap();

    drop(alloc);

    println!("num: {allocated_num}");
}
