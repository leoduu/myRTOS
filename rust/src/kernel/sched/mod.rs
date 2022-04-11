
pub mod round_robin;
mod idle;

use crate::kernel::sync::Mutex;
use crate::utilities::intrusive_linkedlist::ListPtr;

const OS_TICK_PER_SECOND: u64 = 1000;     // secudler every 1ms

use round_robin::RoundRobinScheduler as KernelScheduler;

static SCHEDULER: Mutex<KernelScheduler> = Mutex::new(KernelScheduler::new());

pub trait Scheduler {
    fn run(&self) -> !;

    fn push(&self, thread: ListPtr);
    fn pop(&self) -> Option<ListPtr>;
    fn detach(&self, node: ListPtr);
    
    unsafe fn schedule(&mut self);
}  

pub fn scheduler() -> &'static Mutex<KernelScheduler> {
    &SCHEDULER
}

pub unsafe fn init() {
    idle::init()
}
