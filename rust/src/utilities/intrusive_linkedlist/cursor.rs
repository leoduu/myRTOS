
use core::marker::PhantomData;

use super::list::LinkPtr;
use super::IntrusiveLinkedList;

pub struct Cursor<T: IntrusiveLinkedList<T>> {
    list: LinkPtr,
    owner: PhantomData<T>,
}

impl<T: IntrusiveLinkedList<T>> Cursor<T> {

    #[inline(always)]
    pub fn move_next(&mut self) {
        if let Some(node) = self.list {
            unsafe{ self.list = node.as_ref().next() }
        }
    }

    #[inline(always)]
    pub fn move_prev(&mut self) {
        if let Some(node) = self.list {
            unsafe{ self.list = node.as_ref().prev() }
        }
    }

    #[inline(always)]
    pub fn owner_ref(&self) -> Option<&T> {
        unsafe {T::from_ptr(&self.list)}
    }

}


impl<T: IntrusiveLinkedList<T>> core::convert::From<LinkPtr> for Cursor<T> {
    #[inline(always)]
    fn from(list: LinkPtr) -> Self {
        Self {
            list,
            owner: PhantomData
        }
    }
}

pub struct CursorMut<T: IntrusiveLinkedList<T>> {
    list: LinkPtr,
    owner: PhantomData<T>,
}

impl<T: IntrusiveLinkedList<T>> CursorMut<T> {

    #[inline(always)]
    pub fn insert_before(&mut self, insert_node: LinkPtr) {
        if let Some(node) = &mut self.list {
            unsafe{ node.as_mut().insert_before(insert_node)}
        }
    }

    #[inline(always)]
    pub fn insert_after(&mut self, insert_node: LinkPtr) {
        if let Some(node) = &mut self.list {
            unsafe{ node.as_mut().insert_after(insert_node)}
        }
    }

}


