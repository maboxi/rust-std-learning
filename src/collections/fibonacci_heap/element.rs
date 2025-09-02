use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use inner::*;

use crate::collections::fibonacci_heap::HeapKey;

pub type FibonacciHeapElementWrapper<K, T> = Rc<FibonacciHeapElementInner<K, T>>;
pub type FibonacciHeapElementPointer<K, T> = Option<FibonacciHeapElement<K, T>>;

pub struct FibonacciHeapElement<K: HeapKey, T>(pub(super) FibonacciHeapElementWrapper<K, T>);

impl<K: HeapKey, T> Clone for FibonacciHeapElement<K, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<K: HeapKey, T> Deref for FibonacciHeapElement<K, T> {
    type Target = FibonacciHeapElementInner<K, T>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<K: HeapKey, T> DerefMut for FibonacciHeapElement<K, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(Rc::as_ptr(&self.0) as *mut _) }
    }
}

pub mod inner {
    use std::{cell::RefCell, rc::Weak};

    use crate::collections::fibonacci_heap::inner::FibonacciHeapInner;

    use super::*;

    impl<K: HeapKey, T> FibonacciHeapElement<K, T> {
        pub fn new(
            key: K,
            data: T,
            heap_ref: Weak<RefCell<FibonacciHeapInner<K, T>>>,
        ) -> FibonacciHeapElement<K, T> {
            let mut elem = Self(Rc::new(FibonacciHeapElementInner::new(key, data, heap_ref)));

            let elem_clone = elem.clone();
            elem.right.write(elem_clone.clone());
            elem.left.write(elem_clone);

            elem
        }
    }

    #[allow(unused)]
    pub struct FibonacciHeapElementInner<K: HeapKey, T> {
        pub(crate) data: T,
        pub(crate) key: K,

        pub(crate) degree: usize,
        pub(crate) mark: bool,

        pub(crate) parent: FibonacciHeapElementPointer<K, T>,
        pub(crate) child: FibonacciHeapElementPointer<K, T>,
        pub(crate) right: MaybeUninit<FibonacciHeapElement<K, T>>,
        pub(crate) left: MaybeUninit<FibonacciHeapElement<K, T>>,

        pub(crate) heap_ref: Weak<RefCell<FibonacciHeapInner<K, T>>>,
    }

    impl<K: HeapKey, T> FibonacciHeapElementInner<K, T> {
        fn new(key: K, data: T, heap_ref: Weak<RefCell<FibonacciHeapInner<K, T>>>) -> Self {
            Self {
                key,
                data,
                degree: 0,
                mark: false,
                parent: None,
                child: None,
                right: MaybeUninit::uninit(),
                left: MaybeUninit::uninit(),
                heap_ref,
            }
        }
    }

    impl<K: HeapKey, T> PartialEq for FibonacciHeapElement<K, T> {
        fn eq(&self, other: &Self) -> bool {
            self.key.eq(&other.key)
        }
    }

    impl<K: HeapKey, T> PartialOrd for FibonacciHeapElement<K, T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.key.partial_cmp(&other.key)
        }
    }
}

pub mod reference {
    use std::cell::RefCell;

    use crate::collections::fibonacci_heap::{
        error::HeapReferenceError, inner::FibonacciHeapInner,
    };

    use super::*;

    pub struct FibHeapRef<K: HeapKey, T> {
        element: FibonacciHeapElement<K, T>,
    }
    impl<K: HeapKey, T> FibHeapRef<K, T> {
        pub fn from_elem(elem: &FibonacciHeapElement<K, T>) -> Self {
            Self {
                element: elem.clone(),
                heap: heap_ref,
        }
        }
    }
}
