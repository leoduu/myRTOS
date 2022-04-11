
use core::cell::Cell;

pub struct ListNode<'a, T: 'a + ?Sized> {
    elem: Cell<Option<&'a T>>,
}

pub trait ListNodeInterface<'a, T: 'a + ?Sized> {
    fn next(&'a self) -> &'a ListNode<'a, T>;
}

pub struct LinkedList<'a, T: 'a + ?Sized + ListNodeInterface<'a, T>> {
    head: ListNode<'a, T>,
}

pub struct ListIterator<'a, T: 'a + ?Sized + ListNodeInterface<'a, T>> {
    curr: Option<&'a T>,
}

impl<'a, T: 'a + ?Sized + ListNodeInterface<'a, T>> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.curr {
            Some(res) => {
                self.curr = res.next().elem.get();
                Some(res)
            }
            None => None,
        }
    }
}

impl<'a, T:'a + ?Sized + ListNodeInterface<'a, T>> LinkedList<'a, T>  {
    pub const fn new() -> Self {
        Self {
            head: ListNode::new()
        }
    }

    pub fn head(&self) -> Option<&'a T> {
        self.head.elem.get()
    }

    pub fn push_front(&self, node: &'a T) {
        node.next().elem.set(self.head.elem.get());
        self.head.elem.set(Some(node));
    }

    pub fn push_back(&self, node: &'a T) {
        node.next().elem.set(None);

        match self.iter().last() {
            Some(tail) => tail.next().elem.set(Some(node)),
            None => self.push_front(node),
        }
    }

    pub fn pop_front(&self) -> Option<&'a T> {
        let remove = self.head.elem.get();
        match remove {
            Some(node) => self.head.elem.set(node.next().elem.get()),
            None => self.head.elem.set(None),
        }
        remove
    }

    pub fn iter(&self) -> ListIterator<'a, T> {
        ListIterator {
            curr: self.head.elem.get()
        }
    }

}

impl<'a, T: 'a + ?Sized> ListNode<'a, T> {
    pub const fn new() -> Self {
        Self {
            elem: Cell::new(None),
        }
    }
}


// impl<'a, T: ?Sized> ListLink<'a, T>{
//     pub const fn empty() -> ListLink<'a, T> {
//         ListLink(Cell::new(None))
//     }
// }

// pub trait ListNode<'a ,T: ?Sized> {
//     fn next(&'a self) -> &'a ListLink<'a, T>;
// }

// pub struct List<'a, T: 'a + ?Sized + ListNode<'a, T>> {
//     head: ListLink<'a ,T>,
// }

// pub struct ListIterator<'a, T: 'a + ?Sized + ListNode<'a, T>> {
//     current: Option<&'a T>,
// }

// impl<'a, T: 'a + ?Sized + ListNode<'a, T>> Iterator for ListIterator<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<&'a T> {
//         match self.current {
//             Some(result) => {
//                 self.current = result.next().0.get();
//                 Some(result)
//             }
//             None => None,
//         }
//     }
// }


// impl<'a, T: 'a + ?Sized + ListNode<'a, T>>  List<'a, T> {

//     pub fn iter(&self) -> ListIterator<'a, T> {
//         ListIterator {
//             current: self.head.0.get()
//         }
//     }
// }


