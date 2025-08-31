use std::{mem::MaybeUninit, rc::Weak};

use super::*;
pub struct FibonacciHeapInner<T: FibHeapStorable> {
    h_min: FibonacciHeapElementPointer<T>,
    size: usize,

    // This is basically a weak reference to self
    // need this for creation of FibRef's, which need a weak ref to allow e.g. DecreaseKey without any lookup operation
    pub(in crate::collections::fibonacci_heap) self_ref:
        MaybeUninit<Weak<RefCell<FibonacciHeapInner<T>>>>,

    name: String,
}

#[allow(unreachable_code)]
impl<T: FibHeapStorable> FibonacciHeapInner<T> {
    pub fn new(name: impl Into<String>) -> FibonacciHeap<T> {
        /*
            Definition reference: MakeHeap()

            Theoretical Procedure:
                T.1) Self { .min = NULL, .n = 0 }

            Implementation:
                1) Create new self (see T.1), leave self_ref empty
                2) Wrap new self into pub FibonacciHeap wrapper struct -> FibonacciHeap(self)
                3) insert weak ref to FibonacciHeap(self) into self_ref
                4) return FibonacciHeap(self)
        */

        // 1) and 2)
        let heap = FibonacciHeap(Rc::new(RefCell::new(Self {
            h_min: None,
            size: 0,
            name: name.into(),
            self_ref: MaybeUninit::uninit(),
        })));

        // 3)
        heap.0.borrow_mut().self_ref.write(Rc::downgrade(&heap.0));

        // 4)
        heap
    }

    pub fn insert(&mut self, value: T) -> FibHeapRef<T> {
        /*
           Definition reference: Insert(H, x) where H = &mut self, x = value

            Theoretical Procedure:
                T.1) Create new heap element x with { .degree = 0, .parent = NULL, .child = NULL, .mark = false}
                T.2) Check self.min == NULL
                    T.2A: self.min == NULL ->
                        T.2A.1) Create new root element list containing only x
                        T.2A.2) Set self.min = x
                    T.2B: self.min != NULL ->
                        T.2B.1) Add x to the root element list
                        T.2B.2) If x.key < self.min.key: self.min = x
                T.3) self.n = self.n + 1

            Implementation:
                1) -> T.1: Create new heap element for data value T
                    The new element has the properties described in T.1; left and right point to itself, creating a basic linked list
                2) Create FibHeapRef, which we will later return to the caller; this enables operations like DecreaseKey without additional lookups
                3) Check if self.h_min is None:
                    3A: self.h_min is None ->
                        3B.1
                    3B: self.h_min is Some ->
                        3B.1) -> T.2B.1: insert new element into root linked list
                        3B.2) -> T.2B.2: if the new elements key is smaller than the current min. elements key, replace self.h_min with a reference to the new element
                4) Increase the internal size counter by 1
                5) return the FibHeapRef created in 2)
        */
        let mut new_elem = FibonacciHeapElement::new(value);

        let new_elem_ref = FibHeapRef::from_elem(
            &new_elem,
            unsafe { self.self_ref.assume_init_ref() }.clone(),
        );

        if let Some(h_min) = &mut self.h_min {
            // todo!("Add new_elem to root list");

            Self::ll_insert_left(h_min, &mut new_elem);

            if &new_elem < h_min {
                let _ = self.h_min.replace(new_elem);
            }
        } else {
            assert!(self.empty());
            // todo!("Create new root list with new_elem!");
            let _ = self.h_min.insert(new_elem);
        }

        self.size += 1;

        new_elem_ref
    }

    pub fn min(&self) -> Option<FibonacciHeapElement<T>> {
        // Definition reference: Minimum(H) where H = &mut self
        self.h_min.as_ref().map(|h_min| h_min.clone())
    }

    pub fn union(&mut self, other: FibonacciHeap<T>) {
        // Definition reference: Minimum(H) where H = &mut self

        if let Some(other) = Rc::into_inner(other.0) {
            let mut other = other.into_inner();
            match &mut self.h_min {
                Some(h_min_self) => {
                    if let Some(mut h_min_other) = other.h_min.take() {
                        Self::ll_insert_left(h_min_self, &mut h_min_other);
                        if *h_min_self < h_min_other {
                            let _ = self.h_min.replace(h_min_other);
                        }
                    }

                    self.size += other.size;
                }
                None => {
                    // self heap is empty
                    if other.h_min.is_some() {
                        let name = self.name.clone();
                        *self = other;
                        self.name = name;
                    }
                }
            }
        }
    }

    pub fn extract_min(&mut self) -> Option<FibonacciHeapElement<T>> {
        self.h_min.take().map(|mut z| {
            // todo!("For all children x of z: add x to root list, x.parent = NULL");
            self.h_min = z.child.take();

            if let Some(child) = &mut self.h_min {
                let mut cur_child = unsafe { &mut *child.right.as_mut_ptr() };
                loop {
                    let _ = cur_child.parent.take();
                    if Rc::<_>::ptr_eq(&cur_child.0, &child.0) {
                        cur_child.parent.take();
                        break;
                    }
                    cur_child = unsafe { &mut *cur_child.right.as_mut_ptr() };
                }
            }

            z
        })
    }

    fn ll_insert_left(
        ll_node: &mut FibonacciHeapElement<T>,
        new_node: &mut FibonacciHeapElement<T>,
    ) {
        unsafe {
            new_node.right.assume_init_drop();
            new_node.left.assume_init_drop();
        };

        let ll_left = unsafe { ll_node.left.assume_init_read() };
        ll_node.left.write(new_node.clone());

        new_node.right.write(ll_node.clone());
        new_node.left.write(ll_left);
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }
}
