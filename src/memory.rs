// Copyright 2024 TiKV Project Authors. Licensed under Apache-2.0.

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[macro_export]
macro_rules! impl_display_as_debug {
    ($t:ty) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

#[derive(Debug)]
pub struct MemoryQuotaExceeded;

impl std::error::Error for MemoryQuotaExceeded {}

impl_display_as_debug!(MemoryQuotaExceeded);

pub struct MemoryQuota {
    in_use: AtomicUsize,
    capacity: AtomicUsize,
}

pub struct OwnedAllocated {
    allocated: usize,
    from: Arc<MemoryQuota>,
}

impl OwnedAllocated {
    pub fn new(target: Arc<MemoryQuota>) -> Self {
        Self {
            allocated: 0,
            from: target,
        }
    }

    pub fn alloc(&mut self, bytes: usize) -> Result<(), MemoryQuotaExceeded> {
        self.from.alloc(bytes)?;
        self.allocated += bytes;
        Ok(())
    }

    pub fn allocated(&self) -> usize {
        self.allocated
    }

    pub fn source(&self) -> &MemoryQuota {
        &self.from
    }
}

impl Drop for OwnedAllocated {
    fn drop(&mut self) {
        self.from.free(self.allocated)
    }
}

impl MemoryQuota {
    pub fn new(capacity: usize) -> MemoryQuota {
        MemoryQuota {
            in_use: AtomicUsize::new(0),
            capacity: AtomicUsize::new(capacity),
        }
    }

    pub fn in_use(&self) -> usize {
        self.in_use.load(Ordering::Relaxed)
    }

    /// Returns a floating number between [0, 1] presents the current memory
    /// status.
    pub fn used_ratio(&self) -> f64 {
        self.in_use() as f64 / self.capacity() as f64
    }

    pub fn capacity(&self) -> usize {
        self.capacity.load(Ordering::Relaxed)
    }

    pub fn set_capacity(&self, capacity: usize) {
        self.capacity.store(capacity, Ordering::Relaxed);
    }

    pub fn alloc_force(&self, bytes: usize) {
        let mut in_use_bytes = self.in_use.load(Ordering::Relaxed);
        loop {
            let new_in_use_bytes = in_use_bytes + bytes;
            match self.in_use.compare_exchange_weak(
                in_use_bytes,
                new_in_use_bytes,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(current) => in_use_bytes = current,
            }
        }
    }

    pub fn alloc(&self, bytes: usize) -> Result<(), MemoryQuotaExceeded> {
        let capacity = self.capacity.load(Ordering::Relaxed);
        let mut in_use_bytes = self.in_use.load(Ordering::Relaxed);
        loop {
            if in_use_bytes + bytes > capacity {
                return Err(MemoryQuotaExceeded);
            }
            let new_in_use_bytes = in_use_bytes + bytes;
            match self.in_use.compare_exchange_weak(
                in_use_bytes,
                new_in_use_bytes,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return Ok(()),
                Err(current) => in_use_bytes = current,
            }
        }
    }

    pub fn free(&self, bytes: usize) {
        let mut in_use_bytes = self.in_use.load(Ordering::Relaxed);
        loop {
            // Saturating at the numeric bounds instead of overflowing.
            let new_in_use_bytes = in_use_bytes - std::cmp::min(bytes, in_use_bytes);
            match self.in_use.compare_exchange_weak(
                in_use_bytes,
                new_in_use_bytes,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(current) => in_use_bytes = current,
            }
        }
    }
}

