
use crate::info;
use crate::utilities::intrusive_linkedlist::{ListPtr, Intrusive};
use super::*;
use crate::kernel::thread::Thread;
use crate::utilities::NullLock;

const PRIO: u8 = 1;
const STACK_SIZE: usize = 10240;
static mut THREAD_STACK:[u8; STACK_SIZE] = [0; STACK_SIZE];
static THREAD: NullLock<Thread> = NullLock::new(Thread::new());

static mut A :usize = 0;

pub unsafe fn init() {
    THREAD.lock().init("ipc_t", PRIO, entry, 
                    THREAD_STACK.as_ptr() as usize, STACK_SIZE);
}

pub fn entry(ptr: ListPtr) -> ! {

    let thread = unsafe { Thread::from_ptr_mut(&ptr)};

    let mut a = 0;

    loop {

        thread.sleep(Some(1000));

        if a < 5 {

            MBOX.lock().transmit(a as usize).ok();

        } else if a < 7 {

            let mut msg0: [u8; 3] = [a, a+1, a+2];
            MQUEUE.lock().transmit(&mut msg0, 3).ok();

        } else if a < 9 {

            let mut msg1: [u8; 5] = [a, a+1, a+2, a+3, a+4];
            MQUEUE.lock().transmit(&mut msg1, 5).ok();

        } else {
            thread.sleep(None);
        }

        a += 1;
    }
}


pub fn timer_handler() {

    info!("+++");
}

