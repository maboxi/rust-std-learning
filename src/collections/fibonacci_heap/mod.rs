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

pub trait FibHeapStorable {
    type Key: HeapKey;
    fn key(&self) -> Self::Key;
    fn set_key(&mut self, new_key: Self::Key);
}

pub struct FibonacciHeap<T: FibHeapStorable>(Rc<RefCell<FibonacciHeapInner<T>>>);

impl<T: FibHeapStorable> Clone for FibonacciHeap<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: FibHeapStorable> FibonacciHeap<T> {
    pub fn new(name: impl Into<String>) -> FibonacciHeap<T> {
        FibonacciHeapInner::new(name)
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
