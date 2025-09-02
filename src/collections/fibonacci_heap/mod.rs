use std::{cell::RefCell, rc::Rc};

pub mod error;
use error::HeapReferenceError;

pub mod element;
pub use element::reference::FibHeapRef;
use element::*;

mod inner;
use inner::*;

pub trait HeapKey: PartialOrd {}
impl<T: PartialOrd + PartialEq> HeapKey for T {}

pub struct FibonacciHeap<K: HeapKey, T>(Rc<RefCell<FibonacciHeapInner<K, T>>>);

impl<K: HeapKey, T> Clone for FibonacciHeap<K, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<K: HeapKey, T> FibonacciHeap<K, T> {
    pub fn new(name: impl Into<String>) -> FibonacciHeap<K, T> {
        FibonacciHeapInner::new(name)
    }

    pub fn with_inner<F, R>(&mut self, f: F) -> Result<R, HeapReferenceError>
    where
        F: FnOnce(&mut FibonacciHeapInner<K, T>) -> R,
    {
        self.0
            .try_borrow_mut()
            .map(|ref mut heap_ref| f(heap_ref))
            .map_err(|_| HeapReferenceError::RecursiveExclusiveHeapAccess)
    }
}
