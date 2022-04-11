
use crate::println;
use crate::kernel::thread::Thread;
use crate::utilities::NullLock;

const STACK_SIZE: usize = 512;
static mut THREAD_STACK:[u8; STACK_SIZE] = [0; STACK_SIZE];
static THREAD: NullLock<Thread> = NullLock::new(Thread::new());

// static SCHED_NODE: RoundRobinNode = unsafe { RoundRobinNode::new(&THREAD) };

pub unsafe fn init() {

    THREAD.lock().init("test2", entry, THREAD_STACK.as_ptr() as usize, STACK_SIZE);

}

pub fn entry() -> ! {

    println!("t2 {:?}", crate::arch::support::cpu_status());

    loop {
        THREAD.lock().timer(1000, timer_handler);
    }
}

pub fn timer_handler() {

    unsafe {
        use super::*;
        if let Some(data) = MSG1.receive() {
            println!("{}", data);
        }
    }

    // println!("#######")

    // println!("{:?}", *crate::kernel::sched::scheduler().lock())
}
