// See https://doc.rust-lang.org/nomicon/vec/vec.html

use log::trace;

use inner::InnerVec;
pub struct Vec<T> {
    inner: InnerVec<T>,
    length: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self {
            inner: InnerVec::new(),
            length: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.check_capacity(1);

        let pointer = self.inner.pointer_at(self.length);

        trace!(
            "Vec: pushing value to {pointer:?}; new size: {}",
            self.length + 1
        );

        unsafe { std::ptr::write(pointer, value) };
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        (self.length > 0)
            .then(|| {
                self.length -= 1;
                let pointer = self.inner.pointer_at(self.length);
                trace!("Vec: popping at {pointer:?}; new size: {}", self.length);
                let value = unsafe { std::ptr::read(pointer) };
                value
            })
            .or_else(|| {
                trace!("Vec: tried to pop but vec is empty!");
                None
            })
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.inner.get_capacity()
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        trace!(
            "Vec: Dropping Vec with {} elements by popping all elements and potentially calling their Drop trait implementations",
            self.length
        );
        let mut num_dropped = 0;
        if self.length > 0 {
            while let Some(_) = self.pop() {
                num_dropped += 1;
            } // handle dropping of stored elements
        }
        trace!("Vec: Done! Dropped {num_dropped} elements");
    }
}

impl<T> Vec<T> {
    fn check_capacity(&mut self, new_elements: usize) {
        if self.length + new_elements > self.inner.get_capacity() {
            self.inner.enlarge();
        }
    }
}

mod inner {
    use log::trace;
    use std::ptr::NonNull;

    pub(super) struct InnerVec<T> {
        pointer: NonNull<T>,
        capacity: usize,
    }

    impl<T> InnerVec<T> {
        pub(super) fn new() -> Self {
            assert!(std::mem::size_of::<T>() != 0, "Not ready to handle ZST");
            Self {
                capacity: 0,
                pointer: NonNull::dangling(),
            }
        }

        pub(super) fn get_capacity(&self) -> usize {
            self.capacity
        }

        pub(super) fn enlarge(&mut self) {
            let new_capacity = if self.capacity == 0 {
                1
            } else {
                2 * self.capacity
            };

            trace!(
                "InnerVec: Enlarging from {} to {}",
                self.capacity, new_capacity
            );

            let old_layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            let new_layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();

            let new_pointer = if self.capacity == 0 {
                unsafe { std::alloc::alloc(new_layout) }
            } else {
                let pointer = self.pointer.as_ptr();
                trace!("InnerVec: reallocating from {pointer:?}...");
                unsafe { std::alloc::realloc(pointer as *mut u8, old_layout, new_layout.size()) }
            };

            trace!("InnerVec: data is now at {new_pointer:?}");

            self.pointer = match NonNull::new(new_pointer as *mut T) {
                Some(pointer) => pointer,
                None => std::alloc::handle_alloc_error(new_layout),
            };

            self.capacity = new_capacity;
        }

        pub(super) fn pointer_at(&self, index: usize) -> *mut T {
            assert!(index < self.capacity);
            unsafe { self.pointer.as_ptr().add(index) }
        }
    }

    impl<T> Drop for InnerVec<T> {
        fn drop(&mut self) {
            (self.capacity > 0).then(|| {
                let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                let pointer = self.pointer.as_ptr();
                trace!("InnerVec: Dropping InnerVec, deallocating data at {pointer:?}");
                unsafe { std::alloc::dealloc(pointer as *mut u8, layout) };
            });
        }
    }

    #[cfg(test)]
    mod test {
        use crate::test::test_repeated;
        use std::ptr::NonNull;

        fn test_init() {
            simple_logger::init_with_level(log::Level::Trace).ok();
        }

        #[test]
        fn push_pop() {
            test_init();
            test_repeated("Vec::push_pop", 100, |_| {
                let mut vec = crate::collections::Vec::new();
                assert_eq!(vec.length(), 0);
                assert_eq!(vec.inner.capacity, 0);
                assert_eq!(vec.inner.pointer, NonNull::dangling());

                vec.push(42);
                assert_eq!(vec.inner.get_capacity(), 1);

                vec.push(43);
                vec.push(44);
                assert_eq!(vec.length(), 3);
                assert_eq!(vec.inner.get_capacity(), 4);

                assert_eq!(vec.pop(), Some(44));
                assert_eq!(vec.pop(), Some(43));
                assert_eq!(vec.length(), 1);

                assert_eq!(vec.pop(), Some(42));
                assert_eq!(vec.length(), 0);
                assert_eq!(vec.inner.capacity, 4);

                assert_eq!(vec.pop(), None);
            });
        }

        #[test]
        fn drop() {
            test_init();
            test_repeated("Vec::drop", 100, |_| {
                let (indicator, droptest) = crate::test::DropTest::new();

                {
                    let mut vec = crate::collections::Vec::new();
                    vec.push(droptest);

                    assert!(indicator.is_alive());
                }

                assert!(!indicator.is_alive());
            });
        }

        #[test]
        fn push_pop_largeamount() {
            test_init();
            test_repeated("push_pop_largeamount", 10, |_| {
                const NUM_ELEMENTS: usize = 1000;

                let mut vec = crate::collections::Vec::new();

                let mut test_vec = std::vec::Vec::new();

                for i in 0..NUM_ELEMENTS {
                    vec.push(i);
                    test_vec.push(i);
                }

                assert_eq!(vec.length(), NUM_ELEMENTS);
                assert_eq!(test_vec.len(), vec.length());

                for i in 0..NUM_ELEMENTS {
                    let res = vec.pop();
                    let test_res = test_vec.pop();
                    assert_eq!(res, test_res);
                    assert_eq!(res, Some(NUM_ELEMENTS - i - 1));
                }

                assert_eq!(vec.pop(), None);
                assert_eq!(test_vec.pop(), None);
            });
        }
    }
}
