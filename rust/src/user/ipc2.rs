
use crate::info;
use crate::utilities::intrusive_linkedlist::{ListPtr, Intrusive};
use crate::kernel::thread::Thread;
use crate::utilities::NullLock;
use super::*;

const PRIO: u8 = 1;
const STACK_SIZE: usize = 1024;
static mut THREAD_STACK:[u8; STACK_SIZE] = [0; STACK_SIZE];
static THREAD: NullLock<Thread> = NullLock::new(Thread::new());

// static SCHED_NODE: RoundRobinNode = unsafe { RoundRobinNode::new(&THREAD) };

pub unsafe fn init() {

    THREAD.lock().init("ipc_r", PRIO, entry, 
                    THREAD_STACK.as_ptr() as usize, STACK_SIZE);

}

pub fn entry(ptr: ListPtr) -> ! {

    let thread = unsafe { Thread::from_ptr_mut(&ptr)};

    loop {
        if let Some(data) = MBOX.lock().receive() {
            info!("[{}]", data);
        }

        let mut buf: [u8; 10] = [0; 10];
        if let Some(len) = MQUEUE.lock().receive(&mut buf) {
            info!("{:?}", &buf[0..len as usize]);
        }

        if buf[0] == 8 {
            thread.sleep(None);
        }
    }
}

pub fn timer_handler() {

    info!("#######")

}
