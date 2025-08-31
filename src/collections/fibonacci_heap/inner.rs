use super::*;
pub struct FibonacciHeapInner<T: HeapKey> {
    h_min: FibonacciHeapElementPointer<T>,
    size: usize,

    name: String,
}
#[allow(unreachable_code)]
impl<T: HeapKey> FibonacciHeapInner<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            h_min: None,
            size: 0,
            name: name.into(),
        }
    }

    pub fn insert(&mut self, value: T, heap_ref: &FibonacciHeap<T>) -> FibHeapRef<T> {
        let mut new_elem = FibonacciHeapElement::new(value);

        let new_elem_ref = FibHeapRef::from_elem(&new_elem, heap_ref);

        if let Some(h_min) = &mut self.h_min {
            // todo!("Add new_elem to root list");

            Self::ll_insert_left(h_min, &mut new_elem);

            if new_elem.key < h_min.key {
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

    pub fn min(&self, heap_ref: &FibonacciHeap<T>) -> Option<FibHeapRef<T>> {
        self.h_min
            .as_ref()
            .map(|h_min| FibHeapRef::from_elem(h_min, heap_ref))
    }

    pub fn union(&mut self, other: FibonacciHeap<T>) {
        //todo!("Append root list of other to own root list");

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

    pub fn extract_min(&mut self) -> Option<T> {
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

            z.key.take().unwrap()
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
