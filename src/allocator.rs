use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use spin::Mutex;

/// A simple bump allocator that hands out memory in a sequential manner
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    /// Creates a new empty bump allocator
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initializes the bump allocator with the given heap bounds
    ///
    /// # Safety
    ///
    /// This method is unsafe because the caller must ensure that the given
    /// memory range is unused and that the heap bounds are valid.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        let alloc_start = align_up(allocator.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return null_mut(),
        };

        if alloc_end > allocator.heap_end {
            null_mut() // out of memory
        } else {
            allocator.next = alloc_end;
            allocator.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut allocator = self.lock();
        allocator.allocations -= 1;

        if allocator.allocations == 0 {
            allocator.next = allocator.heap_start;
        }
    }
}

/// A wrapper around a type to provide locked access
pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

/// Align the given address upwards to alignment
fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        addr
    } else {
        addr - remainder + align
    }
}

// Use a static memory region for the heap
// This is a simple approach that doesn't require page table manipulation
static mut HEAP_MEMORY: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

// Heap size: 100 KB
pub const HEAP_SIZE: usize = 100 * 1024;

#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

pub fn init_heap() {
    unsafe {
        let heap_start = core::ptr::addr_of!(HEAP_MEMORY) as usize;
        ALLOCATOR.lock().init(heap_start, HEAP_SIZE);
    }
}

// Export heap start for informational purposes
pub fn get_heap_start() -> usize {
    unsafe { core::ptr::addr_of!(HEAP_MEMORY) as usize }
}
