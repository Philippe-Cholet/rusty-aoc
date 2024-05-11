use core::alloc::{GlobalAlloc, Layout};
use core::fmt;
use core::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::alloc::System;

struct Allocs<T> {
    allocations: T,
    deallocations: T,
    allocations_zeroed: T,
    reallocations: T,
}

#[derive(Debug)]
pub struct AllocCounter<T> {
    ntimes: Allocs<T>,
    total: Allocs<T>,
}

pub type AllocInfos = AllocCounter<usize>;

pub type CounterAllocator = AllocCounter<AtomicUsize>;

impl<T: fmt::Debug> fmt::Debug for Allocs<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entry(&self.allocations)
            .entry(&self.deallocations)
            .entry(&self.allocations_zeroed)
            .entry(&self.reallocations)
            .finish()
    }
}

impl Allocs<AtomicUsize> {
    const fn new() -> Self {
        Self {
            allocations: AtomicUsize::new(0),
            deallocations: AtomicUsize::new(0),
            allocations_zeroed: AtomicUsize::new(0),
            reallocations: AtomicUsize::new(0),
        }
    }

    fn reset(&self) {
        self.allocations.store(0, SeqCst);
        self.deallocations.store(0, SeqCst);
        self.allocations_zeroed.store(0, SeqCst);
        self.reallocations.store(0, SeqCst);
    }

    fn infos(&self) -> Allocs<usize> {
        Allocs {
            allocations: self.allocations.load(SeqCst),
            deallocations: self.deallocations.load(SeqCst),
            allocations_zeroed: self.allocations_zeroed.load(SeqCst),
            reallocations: self.reallocations.load(SeqCst),
        }
    }
}

impl CounterAllocator {
    pub const fn new() -> Self {
        Self {
            ntimes: Allocs::new(),
            total: Allocs::new(),
        }
    }

    pub fn reset(&self) {
        self.ntimes.reset();
        self.total.reset();
    }

    pub fn infos(&self) -> AllocInfos {
        AllocInfos {
            ntimes: self.ntimes.infos(),
            total: self.total.infos(),
        }
    }
}

unsafe impl Sync for CounterAllocator {}

unsafe impl GlobalAlloc for CounterAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.ntimes.allocations.fetch_add(1, SeqCst);
        self.total.allocations.fetch_add(layout.size(), SeqCst);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.ntimes.deallocations.fetch_add(1, SeqCst);
        self.total.deallocations.fetch_add(layout.size(), SeqCst);
        System.dealloc(ptr, layout);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.ntimes.allocations_zeroed.fetch_add(1, SeqCst);
        self.total
            .allocations_zeroed
            .fetch_add(layout.size(), SeqCst);
        System.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.ntimes.reallocations.fetch_add(1, SeqCst);
        self.total.reallocations.fetch_add(layout.size(), SeqCst);
        System.realloc(ptr, layout, new_size)
    }
}
