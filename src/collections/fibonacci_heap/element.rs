use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use inner::*;

use super::HeapKey;

pub type FibonacciHeapElementWrapper<T> = Rc<FibonacciHeapElementInner<T>>;
pub type FibonacciHeapElementPointer<T> = Option<FibonacciHeapElement<T>>;

pub struct FibonacciHeapElement<T>(pub(super) FibonacciHeapElementWrapper<T>)
where
    T: HeapKey;

impl<T> Clone for FibonacciHeapElement<T>
where
    T: HeapKey,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: HeapKey> Deref for FibonacciHeapElement<T> {
    type Target = FibonacciHeapElementInner<T>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T: HeapKey> DerefMut for FibonacciHeapElement<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(Rc::as_ptr(&self.0) as *mut _) }
    }
}

static RC_COUNTER: usize = 0;

pub mod inner {
    use super::*;

    impl<T: HeapKey> FibonacciHeapElement<T> {
        pub fn new(key: T) -> FibonacciHeapElement<T> {
            let id = RC_COUNTER;
            let mut elem = Self(Rc::new(FibonacciHeapElementInner::new(id, key)));

            let elem_clone = elem.clone();
            elem.right.write(elem_clone.clone());
            elem.left.write(elem_clone);

            elem
        }
    }

    pub struct FibonacciHeapElementInner<T: HeapKey> {
        id: usize,

        pub(in crate::collections::fibonacci_heap) key: Option<T>,

        pub(in crate::collections::fibonacci_heap) degree: usize,
        pub(in crate::collections::fibonacci_heap) mark: bool,

        pub(in crate::collections::fibonacci_heap) parent: FibonacciHeapElementPointer<T>,
        pub(in crate::collections::fibonacci_heap) child: FibonacciHeapElementPointer<T>,
        pub(in crate::collections::fibonacci_heap) right: MaybeUninit<FibonacciHeapElement<T>>,
        pub(in crate::collections::fibonacci_heap) left: MaybeUninit<FibonacciHeapElement<T>>,
    }

    impl<T: HeapKey> FibonacciHeapElementInner<T> {
        fn new(id: usize, key: T) -> Self {
            Self {
                id,
                key: Some(key),
                degree: 0,
                mark: false,
                parent: None,
                child: None,
                right: MaybeUninit::uninit(),
                left: MaybeUninit::uninit(),
            }
        }
    }

    impl<T: HeapKey> PartialEq for FibonacciHeapElement<T> {
        fn eq(&self, other: &Self) -> bool {
            self.key.eq(&other.key)
        }
    }

    impl<T> PartialOrd for FibonacciHeapElement<T>
    where
        T: HeapKey,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.key.partial_cmp(&other.key)
        }
    }
}

pub mod reference {
    use std::{cell::RefCell, rc::Weak};

    use crate::collections::fibonacci_heap::{FibonacciHeap, FibonacciHeapInner};

    use super::*;

    pub struct FibHeapRef<T: HeapKey> {
        element: FibonacciHeapElement<T>,
        heap: Weak<RefCell<FibonacciHeapInner<T>>>,
    }

    impl<T: HeapKey> FibHeapRef<T> {
        pub fn from_elem(elem: &FibonacciHeapElement<T>, heap: &FibonacciHeap<T>) -> Self {
            Self {
                element: elem.clone(),
                heap: Rc::downgrade(&heap.0),
            }
        }
    }
}
