use std::{
    marker::PhantomData,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

/*
    Standard library: https://doc.rust-lang.org/std/sync/struct.Arc.html
*/

struct ArcInner<T> {
    value: T,
    ref_count_strong: AtomicUsize,
}

pub struct Arc<T> {
    inner: NonNull<ArcInner<T>>,
    _marker: PhantomData<T>,
}

impl<T> Arc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(ArcInner {
            value,
            ref_count_strong: AtomicUsize::new(1),
        });

        Self {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let inner_ref = unsafe { self.inner.as_ref() };
        inner_ref.ref_count_strong.fetch_add(1, Ordering::Relaxed);

        Self {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // SAFETY:
        let inner_ref = unsafe { self.inner.as_ref() };

        let refcount_strong = inner_ref.ref_count_strong.load(Ordering::Relaxed);

        if refcount_strong == 1 {
            // SAFETY: since we read the refcount atomically, we can be sure that we are the only remaining Arc
            // See Rc Drop impl

            #[allow(dropping_references)]
            drop(inner_ref);
            drop(unsafe { Box::from_raw(self.inner.as_ptr()) })
        } else {
            inner_ref.ref_count_strong.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sync::Arc;
    use std::sync::atomic::Ordering;

    #[test]
    fn arc_test_1() {
        let a = Arc::new(5);
        let b = a.clone();
        let c = b.clone();
        assert_eq!(*a, 5);
        assert_eq!(*b, 5);
        assert_eq!(*c, 5);

        let a_inner_ref = unsafe { a.inner.as_ref() };
        assert_eq!(a_inner_ref.ref_count_strong.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn arc_test_drop() {
        let a = Arc::new(5);
        let b = a.clone();
        let c = b.clone();

        let a_inner_ref = unsafe { a.inner.as_ref() };
        assert_eq!(a_inner_ref.ref_count_strong.load(Ordering::Relaxed), 3);

        drop(c);
        let a_inner_ref = unsafe { a.inner.as_ref() };
        assert_eq!(a_inner_ref.ref_count_strong.load(Ordering::Relaxed), 2);

        drop(b);
        let a_inner_ref = unsafe { a.inner.as_ref() };
        assert_eq!(a_inner_ref.ref_count_strong.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn arc_test_mutex() {
        use std::sync::Mutex;
        let a = Arc::new(Mutex::new(5));
        let b = a.clone();
        let c = b.clone();

        {
            let mut num = a.lock().unwrap();
            *num += 1;
        }

        {
            let mut num = b.lock().unwrap();
            *num += 1;
        }

        {
            let mut num = c.lock().unwrap();
            *num += 1;
        }

        let num = a.lock().unwrap();
        assert_eq!(*num, 8);
    }
}
