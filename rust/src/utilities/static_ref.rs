//! Wrapper type for safe pointers to static memory.

use core::ops::Deref;
use core::marker::PhantomData;

/// A pointer to statically allocated mutable data such as memory mapped I/O
/// registers.
///
/// This is a simple wrapper around a raw pointer that encapsulates an unsafe
/// dereference in a safe manner. It serve the role of creating a `&'static T`
/// given a raw address and acts similarly to `extern` definitions, except
/// `StaticRef` is subject to module and crate boundaries, while `extern`
/// definitions can be imported anywhere.
#[derive(Debug)]
pub struct StaticRef<T> {
    addr: usize,
    phantom: PhantomData<T>,
}

impl<T> StaticRef<T> {
    /// Create a new `StaticRef` from a raw pointer
    ///
    /// ## Safety
    ///
    /// Callers must pass in a reference to statically allocated memory which
    /// does not overlap with other values.
    pub const unsafe fn new(addr: usize) -> Self {
        Self { 
            addr, 
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for StaticRef<T> {
    fn clone(&self) -> Self {
        StaticRef { addr: self.addr, phantom: self.phantom }
    }
}

impl<T> Copy for StaticRef<T> {}

impl<T> Deref for StaticRef<T> {
    type Target = T;
    fn deref(&self) -> &'static T {
        unsafe { &*(self.addr as *const T)  }
    }
}
