use std::cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut};
use std::sync::{LockResult, Mutex, MutexGuard, TryLockResult};

pub trait InnerMutAdapter<T: Clone> {
    fn borrow(&self) -> Ref<T>;
    fn borrow_mut(&self) -> RefMut<T>;
    fn try_borrow(&self) -> Result<Ref<T>, BorrowError>;
    fn try_borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError>;
    fn lock(&self) -> LockResult<MutexGuard<T>>;
    fn try_lock(&self) -> TryLockResult<MutexGuard<T>>;
}

pub struct InnerMut<T: Clone> {
    cell: RefCell<T>,
    mutex: Mutex<T>,
}

impl<T: Clone> InnerMut<T> {
    pub fn new(value: T) -> InnerMut<T> {
        let cell = RefCell::new(value.clone());
        let mutex = Mutex::new(value);

        InnerMut { cell, mutex }
    }
}

impl<T: Clone> InnerMutAdapter<T> for InnerMut<T> {
    fn borrow(&self) -> Ref<T> {
        self.cell.borrow()
    }

    fn borrow_mut(&self) -> RefMut<T> {
        self.cell.borrow_mut()
    }

    fn try_borrow(&self) -> Result<Ref<T>, BorrowError> {
        self.cell.try_borrow()
    }

    fn try_borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        self.cell.try_borrow_mut()
    }

    fn lock(&self) -> LockResult<MutexGuard<T>> {
        self.mutex.lock()
    }

    fn try_lock(&self) -> TryLockResult<MutexGuard<T>> {
        self.mutex.try_lock()
    }
}
