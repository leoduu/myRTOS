
mod idle;

// mod round_robin;
// use round_robin::RoundRobinScheduler as KernelScheduler;

mod preptive_priority;
use preptive_priority::PreptivePriorityScheduler as KernelScheduler;

use crate::utilities::intrusive_linkedlist::ListPtr;

const OS_TICK_PER_SECOND: u64 = 1000;     // secudler every 1ms

static mut SCHEDULER: KernelScheduler = KernelScheduler::new();

pub trait Scheduler {
    fn run(&mut self) -> !;
    fn push(&mut self, thread: ListPtr);
    fn pop(&mut self) -> Option<ListPtr>;
    fn detach(&mut self, node: ListPtr);
    
    unsafe fn schedule(&mut self);
}  

pub fn user_scheduler() {
    use crate::kernel::syscall::*;
    // schedule
    syscall2_0(SysCallClass::Command, 
            SysCallCammand::Schedule as usize);
}

pub fn scheduler() -> &'static mut KernelScheduler {
    unsafe { &mut SCHEDULER }
}

pub unsafe fn init() {
    idle::init()
}
