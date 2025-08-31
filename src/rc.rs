use crate::cell::Cell;
use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

/*
    Standard library: https://doc.rust-lang.org/std/rc/struct.Rc.html
*/

struct RcInner<T> {
    value: T,
    ref_count_strong: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

// SAFETY: Rc: !Send is true because NonNull: !Send

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            ref_count_strong: Cell::new(1),
        });
        Self {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner_ref = unsafe { self.inner.as_ref() };
        inner_ref
            .ref_count_strong
            .set(inner_ref.ref_count_strong.get() + 1);
        Self {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner_ref = unsafe { self.inner.as_ref() };

        let refcount_strong = inner_ref.ref_count_strong.get();

        if refcount_strong == 1 {
            // SAFETY:
            //  We are the last RC, therefore we hold the only reference to the stored value
            //  We are also currently being dropped, therefore when this function returns there won't be any references left to the inner value
            //  Therefore, it is up to us to destroy / drop the stored value, and it is safe to do

            // SAFETY: drop the inner_ref to ensure that we don't use it anymore after we drop the value itself
            #[allow(dropping_references)]
            drop(inner_ref);
            drop(unsafe { Box::from_raw(self.inner.as_ptr()) })
        } else {
            // SAFETY:
            //  There is at least one more Rc referencing the same value, therefore we just need to update the reference counter
            inner_ref.ref_count_strong.set(refcount_strong - 1);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cell::Cell;
    use crate::rc::Rc;

    #[test]
    fn rc_test_1() {
        let rc = Rc::new(Cell::new(42));

        let rc_1 = rc.clone();
        let rc_2 = rc.clone();

        assert_eq!(rc_1.get(), 42);
        assert_eq!(rc_2.get(), 42);

        assert_eq!(unsafe { rc.inner.as_ref() }.ref_count_strong.get(), 3);
    }

    #[test]
    fn rc_test_drop() {
        let (indicator, droptest) = crate::test::DropTest::new();
        let rc = Rc::new(droptest);

        assert!(indicator.is_alive());

        let rc_1 = rc.clone();

        drop(rc);
        assert!(indicator.is_alive());

        drop(rc_1);
        assert!(!indicator.is_alive());
    }
}
