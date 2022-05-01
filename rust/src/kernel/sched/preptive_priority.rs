
use crate::utilities::intrusive_linkedlist::*;
use crate::arch::context;
use crate::kernel::thread::Thread;
use core::fmt;
use core::cell::Cell;

const PRIORITY_NUMBER: usize = 32;

pub struct PreptivePriorityScheduler {
    list: [LinkedList; PRIORITY_NUMBER],
    current: Cell<Option<ListPtr>>,
    prio_mask: u32,
    status: bool,
}

impl PreptivePriorityScheduler {

    pub const fn new() -> Self {

        PreptivePriorityScheduler {
            list: [LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), 
                    LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), 
                    LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), 
                    LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), 
                    LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), 
                    LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), 
                    LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new(), 
                    LinkedList::new(), LinkedList::new(), LinkedList::new(), LinkedList::new()],
            current: Cell::new(None),
            prio_mask: 0,
            status: false,
        }
    }

    fn hightest_priority(&self) -> u8 {

        let mut mask: u32 = 1;
        for i in 0..PRIORITY_NUMBER {
            if mask & self.prio_mask != 0 {
                return i as u8;
            }
            mask = mask << 1;
        }
        panic!()
    }

// }

// impl PreptivePriorityScheduler {
    pub fn run(&mut self) -> ! {

        let prio = self.hightest_priority() as usize;

        if let Some(ptr) = self.list[prio].head() {
            self.status = true;
            self.current.set(Some(ptr));
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

    pub fn add(&mut self, ptr: ListPtr) {
        let prio = unsafe { Thread::from_ptr(&ptr).prio as usize };
        self.list[prio].push_back(ptr);
        self.prio_mask |= 1 << prio; 

        if self.status { unsafe {
            self.schedule()
        }}
    }

    pub fn detach(&mut self, ptr: ListPtr) {

        let prio = unsafe { Thread::from_ptr(&ptr).prio as usize };

        self.list[prio].detach(ptr);

        if self.list[prio].is_empty() {
            self.prio_mask &= !(1 << prio);
        } 
    }

    #[inline(never)]
    pub unsafe fn schedule(&mut self) {

        let list = &self.list[self.hightest_priority() as usize];

        let next = list.pop_front().unwrap();
        let curr = self.current.get().unwrap();

        if curr != next {

            let curr_thread = Thread::from_ptr_mut(&curr);
            let next_thread = Thread::from_ptr_mut(&next);

            // use crate::drivers::timer::system_timer::SysTimerDriver;
            // let tick = crate::board::system_timer().tick();
            // use crate::{print, println};
            // print!("tick[{}] ", tick);
            // println!("{} -> {}", curr_thread, next_thread);

            // crate::println!("{}", self);
            
            list.push_back(next);
            self.current.set(Some(next));
            context::context_switch(curr_thread.sp(), next_thread.sp())
            
        } else {
            list.push_back(next);
            // crate::println!("one thread {}", Thread::from_ptr_mut(&curr));
        } 
    }

}

impl fmt::Display for PreptivePriorityScheduler {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        writeln!(f, "##### Preptive Priority Scheduler #####")?;

        for prio in 0..PRIORITY_NUMBER {

            if self.list[prio].is_empty() {
                continue;
            }

            write!(f, "[{:#02}] :", prio)?;
            
            let mut node = self.list[prio].head();
            while let Some(ptr) = node {
                let thread = unsafe { Thread::from_ptr(&ptr) }; 
                write!(f, "{} ", thread)?;
                node = ptr.next();
            }
            writeln!(f)?;
        }

        Ok(())
        // writeln!(f, "\n{} threads in list", cnt)
    }  
}

impl fmt::Debug for PreptivePriorityScheduler {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        writeln!(f, "##### Preptive Priority Scheduler #####")?;

        for prio in 0..PRIORITY_NUMBER {

            if self.list[prio].is_empty() {
                continue;
            }

            write!(f, "[{:#02}] :", prio)?;
            
            let mut node = self.list[prio].head();
            while let Some(ptr) = node {
                let thread = unsafe { Thread::from_ptr(&ptr) }; 
                write!(f, "{:?} ", thread)?;
                node = ptr.next();
            }
            writeln!(f)?;
        }

        Ok(())
        // writeln!(f, "{} threads in list", cnt)
    }  
}
