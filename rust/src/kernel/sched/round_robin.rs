
use crate::utilities::intrusive_linkedlist::*;
use crate::arch::context;
use crate::kernel::thread::{Thread, ThreadStatus};
use core::fmt;
use super::Scheduler;

pub struct RoundRobinScheduler {
    list: LinkedList,
}

impl RoundRobinScheduler {

    pub const fn new() -> Self {
        RoundRobinScheduler {
            list: LinkedList::new(),
        }
    }

}

impl Scheduler for RoundRobinScheduler {
    fn run(&self) -> ! {
        if let Some(ptr) = self.list.head() {
            unsafe {
                let thread = Thread::from_ptr_mut(&ptr);
                context::context_switch_interrupt(thread.sp());
                cortex_m::interrupt::enable();
            }
        } else {
            panic!("no thread to run");
        }

        // never reach here
        loop {}
    }

    #[inline]
    fn push(&self, node: ListPtr) {
        self.list.push_back(node);
    }

    #[inline]
    fn pop(&self) -> Option<ListPtr> {
        self.list.pop_front()
    }

    fn detach(&self, node: ListPtr) {

        if let Some(head) = self.list.head() {
            if node == head {
                self.pop();
            } else {
                self.list.detach(node);
            }
        }
    }

    #[inline(never)]
    unsafe fn schedule(&mut self) {


        let curr = self.pop().unwrap();
        self.push(curr);

        let mut node = self.list.head().unwrap();

        loop {
  
            let thread = unsafe { Thread::from_ptr_mut(&node) };

            if thread.status == ThreadStatus::Ready {
                break;
            } else {

                node = node.next().unwrap();
                let a = self.pop().unwrap();
                self.push(a);
            }
        }

        if curr != node { unsafe {

            let curr_thread = Thread::from_ptr_mut(&curr);
            let next_thread = Thread::from_ptr_mut(&node);

            // use crate::drivers::timer::system_timer::SysTimerDriver;
            // let tick = crate::board::system_timer().tick();
            // use crate::{print, println};
            // print!("tick[{}] ", tick); curr_thread.show_name(); 
            // print!(" -> "); next_thread.show_name(); println!();
            // println!("{}", self);

            context::context_switch(curr_thread.sp(), next_thread.sp())
            
        }} else {
            // use crate::println;
            // println!("one thread");
        } 
    }

}

impl fmt::Display for RoundRobinScheduler {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        writeln!(f, "\n##### Round Robin Scheduler #####")?;

        let mut ptr = self.list.head().unwrap();
        let mut cnt = 1;

        unsafe { loop {
            let thread = Thread::from_ptr(&ptr); 

            if thread.status == ThreadStatus::Ready {
                for i in 0..10 {
                    write!(f, "{}", thread.name[i] as char)?;
                }
                write!(f, " ")?;
            }
            if let Some(next) = ptr.next() {
                ptr = next;
                cnt += 1;
            } else {
                break;
            }
        }}

        writeln!(f, "\n{} threads in list", cnt)
    }  
}

impl fmt::Debug for RoundRobinScheduler {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        writeln!(f, "\n##### Round Robin Scheduler #####")?;

        let mut ptr = self.list.head().unwrap();
        let mut cnt = 1;

        unsafe { loop {
            write!(f, "{:?}", Thread::from_ptr(&ptr))?;
            if let Some(next) = ptr.next() {
                ptr = next;
                cnt += 1;
            } else {
                break;
            }
        }}

        writeln!(f, "{} threads in list", cnt)
    }  
}
