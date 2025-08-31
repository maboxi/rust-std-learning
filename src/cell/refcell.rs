use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

/*
    Standard library: https://doc.rust-lang.org/std/cell/struct.RefCell.html
*/

use crate::cell::Cell;

#[derive(Copy, Clone)]
enum RefCellState {
    Unshared,
    Exclusive,
    Shared(usize),
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefCellState>,
}

// SAFETY: RefCell: !Sync is true since RefCell contains UnsafeCell, and UnsafeCell: !Sync

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefCellState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        // SAFETY: We only give out a Ref if no RefMut has been given out yet
        //  By updating self.state, we keep track of the number of Ref's given out
        //  When a Ref is dropped, it will decrement the count of existing Ref's by one (or set it to Unshared if it was the last Ref)

        match self.state.get() {
            RefCellState::Unshared => {
                self.state.set(RefCellState::Shared(1));
                Some(Ref { refcell: &self })
            }
            RefCellState::Shared(n) => {
                self.state.set(RefCellState::Shared(n + 1));
                Some(Ref { refcell: &self })
            }
            RefCellState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        // SAFETY: We only give out a RefMut if no Ref or RefMut currently exists
        //  By settings self.state to Exclusive, we prevent the creation of any Ref's or another RefMut until the RefMut given out here is dropped
        //  When a RefMut is dropped, it will reset self.state to Unshared, once again allowing the creation of Ref's or a RefMut
        if let RefCellState::Unshared = self.state.get() {
            self.state.set(RefCellState::Exclusive);
            Some(RefMut { refcell: &self })
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<'refcell, T> Deref for Ref<'refcell, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<'refcell, T> Drop for Ref<'refcell, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefCellState::Shared(1) => self.refcell.state.set(RefCellState::Unshared),
            RefCellState::Shared(n) => self.refcell.state.set(RefCellState::Shared(n - 1)),
            RefCellState::Exclusive | RefCellState::Unshared => unreachable!(),
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<'refcell, T> Deref for RefMut<'refcell, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<'refcell, T> DerefMut for RefMut<'refcell, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<'refcell, T> Drop for RefMut<'refcell, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefCellState::Exclusive => self.refcell.state.set(RefCellState::Unshared),
            RefCellState::Unshared | RefCellState::Shared(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create() {
        let refcell = RefCell::new(42);

        assert_eq!(42, unsafe { *refcell.value.get() })
    }

    #[test]
    fn borrow() {
        let refcell = RefCell::new(42);

        let borrow_1 = refcell.borrow().unwrap();
        let borrow_2 = refcell.borrow().unwrap();

        assert_eq!(*borrow_1, 42);
        assert_eq!(*borrow_2, 42);

        assert!(refcell.borrow_mut().is_none());

        drop(borrow_1);
        drop(borrow_2);

        assert!(refcell.borrow_mut().is_some());
    }

    #[test]
    fn borrow_mut() {
        let refcell = RefCell::new(42);

        assert_eq!(*refcell.borrow().unwrap(), 42);

        let mut borrow_mut_1 = refcell.borrow_mut().unwrap();
        assert!(refcell.borrow_mut().is_none());

        *borrow_mut_1 = 69;

        drop(borrow_mut_1);
        assert!(refcell.borrow_mut().is_some());
        assert_eq!(*refcell.borrow_mut().unwrap(), 69);
        assert_eq!(*refcell.borrow().unwrap(), 69);
    }
}
