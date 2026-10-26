#![allow(dead_code)]
// SigmaOS Kernel Library Collections
// Eliminates dependency on std::collections

use core::cell::Cell;

pub use super::btreemap::BTreeMap;
pub use super::btreemap::BTreeMap as SigmaBTreeMap;
pub use super::hashmap::{Entry, HashMap};
pub use super::hashset::HashSet;
pub use super::vecdeque::VecDeque;

/// Simple atomic types for klib (no_std compatible)
pub struct AtomicBool {
    value: Cell<bool>,
}

impl AtomicBool {
    pub fn new(v: bool) -> Self {
        Self {
            value: Cell::new(v),
        }
    }

    pub fn load(&self) -> bool {
        self.value.get()
    }

    pub fn store(&self, v: bool) {
        self.value.set(v);
    }

    pub fn swap(&self, v: bool) -> bool {
        self.value.replace(v)
    }
}

pub struct AtomicUsize {
    value: Cell<usize>,
}

impl AtomicUsize {
    pub fn new(v: usize) -> Self {
        Self {
            value: Cell::new(v),
        }
    }

    pub fn load(&self) -> usize {
        self.value.get()
    }

    pub fn store(&self, v: usize) {
        self.value.set(v);
    }

    pub fn fetch_add(&self, v: usize) -> usize {
        let old = self.value.get();
        self.value.set(old.wrapping_add(v));
        old
    }

    pub fn fetch_sub(&self, v: usize) -> usize {
        let old = self.value.get();
        self.value.set(old.wrapping_sub(v));
        old
    }
}

pub struct AtomicU64 {
    value: Cell<u64>,
}

impl AtomicU64 {
    pub fn new(v: u64) -> Self {
        Self {
            value: Cell::new(v),
        }
    }

    pub fn load(&self) -> u64 {
        self.value.get()
    }

    pub fn store(&self, v: u64) {
        self.value.set(v);
    }

    pub fn fetch_add(&self, v: u64) -> u64 {
        let old = self.value.get();
        self.value.set(old.wrapping_add(v));
        old
    }
}

/// Memory ordering constants (simplified for no_std)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_basic() {
        let mut map = BTreeMap::<u32, u32>::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_hashset_basic() {
        let set: HashSet<u32> = HashSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_vecdeque_basic() {
        let deque: VecDeque<u32> = VecDeque::new();
        assert!(deque.is_empty());
        assert_eq!(deque.len(), 0);
    }

    #[test]
    fn test_atomic_bool() {
        let atomic = AtomicBool::new(false);
        assert!(!atomic.load());
        atomic.store(true);
        assert!(atomic.load());
    }

    #[test]
    fn test_atomic_usize() {
        let atomic = AtomicUsize::new(0);
        assert_eq!(atomic.load(), 0);
        assert_eq!(atomic.fetch_add(5), 0);
        assert_eq!(atomic.load(), 5);
    }

    #[test]
    fn test_atomic_u64() {
        let atomic = AtomicU64::new(0);
        assert_eq!(atomic.load(), 0);
        assert_eq!(atomic.fetch_add(10), 0);
        assert_eq!(atomic.load(), 10);
    }
}
