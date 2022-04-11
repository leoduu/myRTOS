
use core::fmt;
use core::ptr::NonNull;
use core::cell::Cell;

pub struct LinkedList {
    head: Cell<Option<ListPtr>>,
}

impl LinkedList {
    pub const fn new() -> Self {
        Self {
            head: Cell::new(None),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.get().is_none()
    }

    #[inline]
    pub fn head(&self) -> Option<ListPtr> {
        self.head.get()
    }

    #[inline]
    pub fn is_head(&self, node: ListPtr) -> bool {
        if let Some(head) = self.head() {
            if head.0 == node.0 {
                return true;
            }
        }
        false
    }

    pub fn tail(&self) -> Option<ListPtr> {

        if self.head().is_none() {
            return None;
        }

        let mut curr = self.head().unwrap();

        while let Some(node) = curr.next() {
            curr = node;
        }

        Some(curr)

    }

    pub fn push_front(&self, node: ListPtr) {

        node.set_prev(None);
        node.set_next(self.head());

        if let Some(head) = self.head() {
            head.set_prev(Some(node))
        }
        self.head.set(Some(node));
    }

    pub fn push_back(&self, node: ListPtr) {

        if self.head().is_none() {
            self.head.set(Some(node));
            return;
        }

        node.set_prev(self.tail());
        node.set_next(None);

        if let Some(tail) = self.tail() {
            tail.set_next(Some(node));
        } 
    }

    pub fn pop_front(&self) -> Option<ListPtr> {

        if let Some(head) = self.head() {

            self.head.set( 
                if let Some(new) = head.next() {
                    new.set_prev(None);
                    Some(new)
                } else {
                    None
                }
            );
            head.set_next(None);

            Some(head)
            
        } else {
            None
        }
    }

    pub fn detach(&self, node: ListPtr) {

        if self.is_head(node) {
            self.head.set(node.next());
            if let Some(head) = self.head() {
                head.set_prev(None);
            }
        } else {
            // previous node
            if let Some(prev) = node.prev() {
                prev.set_next(node.next());
            }
            // next node
            if let Some(next) = node.next() {
                next.set_prev(node.prev());
            }
            node.set_next(None);
            node.set_prev(None);
        }
    }

} 


#[derive(Clone, Copy, PartialEq)]
pub struct ListPtr (pub NonNull<ListNode>);

unsafe impl Send for ListPtr {}

impl ListPtr {

    #[inline]
    pub fn new(node: &mut ListNode) -> Option<ListPtr> {
        NonNull::new(node).map(|ptr| ListPtr(ptr))
    }

    #[inline]
    pub unsafe fn new_uncheck(node: &mut ListNode) -> ListPtr {
        ListPtr (NonNull::new_unchecked(node))
    }

    #[inline]
    pub fn prev(&self) -> Option<ListPtr> {
        unsafe {
            self.0.as_ref().prev()
        }
    }

    #[inline]
    pub fn next(&self) -> Option<ListPtr> {
        unsafe {
            self.0.as_ref().next()
        }
    }

    #[inline]
    pub fn set_prev(&self, node: Option<ListPtr>) {
        unsafe {
            self.0.as_ref().set_prev(node)
        }
    }

    #[inline]
    pub fn set_next(&self, node: Option<ListPtr>) {
        unsafe {
            self.0.as_ref().set_next(node)
        }
    }

    pub fn insert(&self, node: ListPtr) {

        node.set_prev(Some(*self));
        node.set_next(self.next());

        if let Some(next) = self.next() {
            next.set_prev(Some(node));
        }
        self.set_next(Some(node));

    }
}

// impl Iterator for ListPtr {
//     type Item = ListPtr;

//     fn next(&mut self) -> Option<Self::Item> {
//         unsafe {
//             self.0.as_ref().next.get().map(|ptr| ListPtr(ptr))
//         }
//     }
// }

impl fmt::Debug for ListPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            writeln!(f, "addr| {:?}", self.0).ok();
            writeln!(f, "  {:?}", self.0.as_ref())
        }
    }
}


pub struct ListNode {
    prev: Cell<Option<ListPtr>>, 
    next: Cell<Option<ListPtr>>,
}


impl ListNode {
    pub const fn new() -> ListNode {         
        ListNode {
            prev: Cell::new(None),
            next: Cell::new(None),
        }
    }
    
    pub fn is_link(&self) -> bool {
        self.prev.get().is_some() &&
        self.next.get().is_some()
    }

    #[inline]
    pub fn next(&self) -> Option<ListPtr> {
        self.next.get()
    }

    #[inline]
    pub fn prev(&self) -> Option<ListPtr> {
        self.prev.get()
    }

    #[inline]
    pub fn set_prev(&self, node: Option<ListPtr>) {
        self.prev.set(node)
    }

    #[inline]
    pub fn set_next(&self, node: Option<ListPtr>) {
        self.next.set(node)
    }

    pub fn insert(&mut self, node: ListPtr) {
        
        node.set_prev(ListPtr::new(self));
        node.set_next(self.next());

        self.set_next(Some(node));
        if let Some(next) = self.next() {
            next.set_prev(Some(node));
        }
    }

}

impl Default for ListNode {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ", self.prev.get().map(|p| p.0))?;
        write!(f, "{:?} ", self as *const _)?;
        write!(f, "{:?} ", self.next.get().map(|p| p.0))
    }
}

