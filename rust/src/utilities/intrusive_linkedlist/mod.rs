
mod linked_list;
pub mod macros;

pub use linked_list::*;
//pub use cursor::Cursor;

pub trait Intrusive {

    type Item;

    unsafe fn from_ptr(ptr: &ListPtr) -> &Self::Item;

    unsafe fn from_ptr_mut(ptr: &ListPtr) -> &mut Self::Item;
}

