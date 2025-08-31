use std::cell::UnsafeCell;

/*
    Standard library: https://doc.rust-lang.org/std/cell/struct.Cell.html
*/

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// SAFETY: Cell: !Sync is true since Cell contains UnsafeCell, and UnsafeCell: !Sync

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we can overwrite the value stored in this cell because
        //  A: we don't give out any references to this value, only copies -> no references can be invalidated
        //  B: since Cell is !Sync, no concurrent access to Cell is possible
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we can read out this value because no one else can currently modify this value (no &mut T to the value inside of .value can exist)
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use std::cell::Cell;

    #[test]
    fn cell_test_all() {
        let cell = Cell::new(42);

        let cell_ref_1 = &cell;
        let cell_ref_2 = &cell;

        assert_eq!(cell.get(), 42);
        assert_eq!(cell_ref_1.get(), 42);
        assert_eq!(cell_ref_2.get(), 42);

        cell_ref_1.set(69);

        assert_eq!(cell.get(), 69);
        assert_eq!(cell_ref_1.get(), 69);
        assert_eq!(cell_ref_2.get(), 69);
    }
}
