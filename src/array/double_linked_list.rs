use std::ptr;

struct DRec<T> {
    item: T,
    next: *mut DRec<T>,
    prev: *mut DRec<T>,
}

pub struct DoubleLinkedList<T> {
    first: *mut DRec<T>,
    last: *mut DRec<T>,
    size: usize,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            first: ptr::null_mut(),
            last: ptr::null_mut(),
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add(&mut self, item: T) {
        self.add_at(item, self.size);
    }

    // pub fn get(&self, index: usize) -> &T {
    //     unsafe { &(*self.find(index)).item }
    // }

    pub fn add_at(&mut self, item: T, index: usize) {
        let new_rec = Box::into_raw(Box::new(DRec {
            item,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }));

        if self.size == 0 {
            self.first = new_rec;
            self.last = new_rec;
        } else if index == 0 {
            unsafe {
                (*new_rec).next = self.first;
                (*self.first).prev = new_rec;
                self.first = new_rec;
            }
        } else if index == self.size {
            unsafe {
                (*new_rec).prev = self.last;
                (*self.last).next = new_rec;
                self.last = new_rec;
            }
        } else {
            unsafe {
                let exist = self.find(index);
                (*new_rec).next = exist;
                (*new_rec).prev = (*exist).prev;
                (*(*exist).prev).next = new_rec;
                (*exist).prev = new_rec;
            }
        }
        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        let rec = self.find(index);
        unsafe {
            if self.size == 1 {
                self.first = ptr::null_mut();
                self.last = ptr::null_mut();
            } else if (*rec).prev.is_null() {
                self.first = (*rec).next;
                (*self.first).prev = ptr::null_mut();
            } else if (*rec).next.is_null() {
                self.last = (*rec).prev;
                (*self.last).next = ptr::null_mut();
            } else {
                (*(*rec).prev).next = (*rec).next;
                (*(*rec).next).prev = (*rec).prev;
            }
            self.size -= 1;
            Box::from_raw(rec).item
        }
    }

    // pub fn reset(&mut self) {
    //     while self.size > 0 {
    //         self.remove(0);
    //     }
    // }

    fn find(&self, index: usize) -> *mut DRec<T> {
        let mut rec;
        let mut ix;
        if index > self.size / 2 {
            rec = self.last;
            ix = self.size - 1;
            while ix != index {
                ix -= 1;
                rec = unsafe { (*rec).prev };
            }
        } else {
            rec = self.first;
            ix = 0;
            while ix != index {
                ix += 1;
                rec = unsafe { (*rec).next };
            }
        }
        rec
    }
}
