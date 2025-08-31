use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use inner::*;

use crate::collections::fibonacci_heap::FibHeapStorable;

pub type FibonacciHeapElementWrapper<T> = Rc<FibonacciHeapElementInner<T>>;
pub type FibonacciHeapElementPointer<T> = Option<FibonacciHeapElement<T>>;

pub struct FibonacciHeapElement<T: FibHeapStorable>(pub(super) FibonacciHeapElementWrapper<T>);

impl<T: FibHeapStorable> Clone for FibonacciHeapElement<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: FibHeapStorable> Deref for FibonacciHeapElement<T> {
    type Target = FibonacciHeapElementInner<T>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T: FibHeapStorable> DerefMut for FibonacciHeapElement<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(Rc::as_ptr(&self.0) as *mut _) }
    }
}

static RC_COUNTER: usize = 0;

pub mod inner {
    use super::*;

    impl<T: FibHeapStorable> FibonacciHeapElement<T> {
        pub fn new(data: T) -> FibonacciHeapElement<T> {
            let id = RC_COUNTER;
            let mut elem = Self(Rc::new(FibonacciHeapElementInner::new(id, data)));

            let elem_clone = elem.clone();
            elem.right.write(elem_clone.clone());
            elem.left.write(elem_clone);

            elem
        }
    }

    pub struct FibonacciHeapElementInner<T: FibHeapStorable> {
        id: usize,

        pub(in crate::collections::fibonacci_heap) data: T,

        pub(in crate::collections::fibonacci_heap) degree: usize,
        pub(in crate::collections::fibonacci_heap) mark: bool,

        pub(in crate::collections::fibonacci_heap) parent: FibonacciHeapElementPointer<T>,
        pub(in crate::collections::fibonacci_heap) child: FibonacciHeapElementPointer<T>,
        pub(in crate::collections::fibonacci_heap) right: MaybeUninit<FibonacciHeapElement<T>>,
        pub(in crate::collections::fibonacci_heap) left: MaybeUninit<FibonacciHeapElement<T>>,
    }

    impl<T: FibHeapStorable> FibonacciHeapElementInner<T> {
        fn new(id: usize, data: T) -> Self {
            Self {
                id,
                data,
                degree: 0,
                mark: false,
                parent: None,
                child: None,
                right: MaybeUninit::uninit(),
                left: MaybeUninit::uninit(),
            }
        }
    }

    impl<T: FibHeapStorable> PartialEq for FibonacciHeapElement<T> {
        fn eq(&self, other: &Self) -> bool {
            self.data.key().eq(&other.data.key())
        }
    }

    impl<T: FibHeapStorable> PartialOrd for FibonacciHeapElement<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.data.key().partial_cmp(&other.data.key())
        }
    }
}

pub mod reference {
    use std::{cell::RefCell, rc::Weak};

    use crate::collections::fibonacci_heap::FibonacciHeapInner;

    use super::*;

    pub struct FibHeapRef<T: FibHeapStorable> {
        element: FibonacciHeapElement<T>,
        heap: Weak<RefCell<FibonacciHeapInner<T>>>,
    }

    impl<T: FibHeapStorable> FibHeapRef<T> {
        pub fn from_elem(
            elem: &FibonacciHeapElement<T>,
            heap_ref: Weak<RefCell<FibonacciHeapInner<T>>>,
        ) -> Self {
            Self {
                element: elem.clone(),
                heap: heap_ref,
            }
        }
    }
}
