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

pub struct FibonacciHeap<T: HeapKey>(Rc<RefCell<FibonacciHeapInner<T>>>);

impl<T: HeapKey> Clone for FibonacciHeap<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: HeapKey> FibonacciHeap<T> {
    pub fn new(name: impl Into<String>) -> FibonacciHeap<T> {
        FibonacciHeap(Rc::new(RefCell::new(FibonacciHeapInner::new(name))))
    }

    pub fn with_inner<F, R>(&mut self, f: F) -> Result<R, HeapReferenceError>
    where
        F: FnOnce(&mut FibonacciHeapInner<T>) -> R,
    {
        match self.0.try_borrow_mut() {
            Ok(ref mut heap) => Ok(f(heap)),
            Err(error) => Err(error.into()),
        }
    }
}
