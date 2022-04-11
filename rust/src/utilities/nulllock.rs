
use core::cell::UnsafeCell;

use core::ops::{Deref, DerefMut};

pub struct NullLock<T: ?Sized> {
    data: UnsafeCell<T>,
}

// Same unsafe impls as `std::sync::NullLock`
unsafe impl<T: ?Sized + Send> Sync for NullLock<T> {}
unsafe impl<T: ?Sized + Send> Send for NullLock<T> {}

impl<T> NullLock<T> {    
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> NullLockGuard<'_, T> {
        // lock
        NullLockGuard {
            lock: self
        }
    }
}

pub struct NullLockGuard<'a, T: ?Sized + 'a> {
    lock: &'a NullLock<T>,
}

impl<T: Sized + Default> Default for NullLock<T> {
    fn default() -> NullLock<T> {
        NullLock::new(Default::default())
    }
}

impl<'a, T: ?Sized> Deref for NullLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T{
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T: ?Sized> DerefMut for NullLockGuard<'a, T> {

    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T: ?Sized> Drop for NullLockGuard<'a, T> {
    // unlock
    fn drop(&mut self) {
        
    }
}
