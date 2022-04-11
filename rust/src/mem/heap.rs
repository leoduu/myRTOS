use core::fmt;
use core::ptr::NonNull;
use core::alloc::Layout;
use core::alloc::GlobalAlloc;

use crate::align_up_16_bit;
use crate::container_of;
use crate::utilities::intrusive_linkedlist::*;
use crate::kernel::sync::Mutex;
use crate::container_of_mut;

const HEAP_NODE_SIZE : usize = core::mem::size_of::<HeapNode>();

pub enum HeapNodeStatus {
    Used,
    Free,
    Tail
}

impl fmt::Display for HeapNodeStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Used => write!(f, "Used"),
            Self::Free => write!(f, "Free"),
            Self::Tail => write!(f, "Tail"),
        }
    }
}

pub struct LockedHeap(Mutex<Heap>);

impl LockedHeap {
    pub const fn new() -> Self {
        LockedHeap(Mutex::new(Heap::new()))
    }

    pub unsafe fn init(&self, bottom: usize, size: usize) {
        self.0
        .lock()
        .init(bottom, size)
    }
}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
        .lock()
        .allocate(layout)
        .ok()
        .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0
        .lock()
        .deallocate(ptr, layout)
    }
}


impl fmt::Debug for LockedHeap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.lock().fmt(f)
    }
}


struct Heap {
    bottom: usize,
    total:  usize,
    used:   usize,
    free:   usize,
    list:   LinkedList,
    // free:   HeapNode
}

unsafe impl Send for Heap {}

impl Heap {
    pub const fn new() -> Self {
        Self { 
            bottom: 0,
            total:  0,
            used:   0,
            free:   0,
            list:   LinkedList::new()
        }
    }

    pub unsafe fn init(&mut self, bootom: usize, size: usize) {
        self.bottom = bootom;
        self.total = size;
        self.free = self.total - 2 * HEAP_NODE_SIZE;
        self.used = 2 * HEAP_NODE_SIZE;
        self.list.push_back(HeapNode::new(self.bottom, self.total).unwrap());

        let tail = HeapNode::new(self.bottom + size - HEAP_NODE_SIZE, 0).unwrap();
        HeapNode::from_ptr_mut(&tail).set_status(HeapNodeStatus::Tail);
        self.list.push_back(tail);
    }

    pub fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {

        // at least allocate 16 Bytes
        let align_size = align_up_16_bit!(layout.pad_to_align().size());

        if let Some(ptr) = self.find_first_free_node(align_size) {

            let free_node = unsafe { HeapNode::from_ptr_mut(&ptr) };

            let remain_size = free_node.size - align_size;
            free_node.status = HeapNodeStatus::Used;
            free_node.size   = align_size;
            
            if remain_size > HEAP_NODE_SIZE + 16 {
                unsafe {
                    let remain_addr = free_node as *mut _ as usize + HEAP_NODE_SIZE + align_size;
                    if let Some(remain_node) = HeapNode::new(remain_addr , remain_size) {
                        free_node.insert_after(remain_node);
                    } else {
                        return Err(())
                    };
                }
            }
            
            let return_addr = free_node as *mut _ as usize + HEAP_NODE_SIZE;
            self.free -= align_size + HEAP_NODE_SIZE;
            self.used += align_size + HEAP_NODE_SIZE;
            return Ok(NonNull::new(return_addr as *mut u8).unwrap());

        } else {
            return Err(());
        }
    }

    pub fn deallocate(&mut self, ptr: *mut u8, _layout: Layout) {

        unsafe {
            let heap_ptr = (ptr as usize - HEAP_NODE_SIZE) as *mut HeapNode;
            let heap_node = &mut *heap_ptr;
            heap_node.status = HeapNodeStatus::Free;
            self.used -= heap_node.size + HEAP_NODE_SIZE;
            self.free += heap_node.size + HEAP_NODE_SIZE;
            self.free += HEAP_NODE_SIZE * heap_node.merge();
        }       
    }

    fn find_first_free_node(&self, size: usize) -> Option<ListPtr> {
        unsafe { 
            
            let mut ptr = self.list.head().unwrap();
            
            loop {
                let heap_node = HeapNode::from_ptr_mut(&ptr);
                
                match heap_node.status {
                    HeapNodeStatus::Free => {
                        if heap_node.size >= size {
                            return Some(ptr);
                        }
                    }
                    _ => {}
                }            
                
                // move next
                if let Some(next) = heap_node.node.next() {
                    ptr = next;
                } else {
                    return None;
                }
            }
        }
    }

}


impl fmt::Debug for Heap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        writeln!(f, "\n============== Heap map ==============")?;
        writeln!(f, "total {}\tused {}  \tfree {}", self.total, self.used, self.free)?;
        if self.list.is_empty() {
            writeln!(f, "\tnothing")?;
            return Ok(())
        }

        let mut ptr = self.list.head().unwrap();
        unsafe { loop {

            let heap_node = HeapNode::from_ptr(&ptr);
            write!(f, "{:?}", heap_node)?; 
                
            if let Some(next) = heap_node.node.next() {
                ptr = next;
            } else {
                break;
            }
        }}
        writeln!(f, "================ tail ================")?;
        Ok(())
    }
}

struct HeapNode {
    node: ListNode,
    status: HeapNodeStatus,
    size: usize,
}

impl HeapNode {
    unsafe fn new(addr: usize, size: usize) -> Option<ListPtr> {
        let ptr = addr as *mut HeapNode;
        ptr.write_volatile(HeapNode {
            node: ListNode::new(),
            status: HeapNodeStatus::Free,
            size,
        });

        ListPtr::new(&mut (*ptr).node)
    }

    #[inline]
    fn set_status(&mut self, status: HeapNodeStatus) {
        self.status = status
    }

    #[inline]
    pub fn insert_after(&mut self, node: ListPtr) {
        self.node.insert(node)
    }

    pub fn merge(&mut self) -> usize {

        let mut n = 0;
        unsafe {
            // merge next
            if let Some(ptr) = self.node.next() {
                let next = HeapNode::from_ptr_mut(&ptr);

                // if next is free :
                // self.next = next.next
                // next.next.prev = self
                if let HeapNodeStatus::Free = next.status {
                    self.node.set_next(next.node.next());
                    if let Some(next_next) = next.node.next() {
                        next_next.set_prev(next.node.prev());
                    }
                    self.size += next.size + HEAP_NODE_SIZE;
                    n += 1;
                }
            }

            // merge prev
            if let Some(ptr) = self.node.prev() {
                let prev = HeapNode::from_ptr_mut(&ptr);

                // if prev is free :
                // prev.next = self.next
                // self.next.prev = prev
                if let HeapNodeStatus::Free = prev.status {
                    prev.node.set_next(self.node.next());
                    if let Some(next) = &mut self.node.next() {
                        next.set_prev(self.node.prev());
                    }
                    prev.size += self.size + HEAP_NODE_SIZE;
                    n += 1;
                }
            }
        }
        n
    }
}

impl Intrusive for HeapNode {
    type Item = HeapNode;

    unsafe fn from_ptr(ptr: &ListPtr) -> &Self::Item {
        container_of!(ptr.0.as_ref(), HeapNode, node)
    }

    unsafe fn from_ptr_mut(ptr: &ListPtr) -> &mut Self::Item {
        container_of_mut!(ptr.0.as_ref(), HeapNode, node)
    }
}

impl fmt::Debug for HeapNode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        writeln!(f, "\tHeapNode {:?} {}", self as *const _, self.status)?;
        writeln!(f, "\t\tsize | {} Bytes", self.size)?;
        writeln!(f, "\t\tprev | {:?}", self.node.prev().map(|p| p.0))?;
        writeln!(f, "\t\tnext | {:?}\n", self.node.next().map(|p| p.0))
    }
}
