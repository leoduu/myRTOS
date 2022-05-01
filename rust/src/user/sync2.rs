
use crate::info;
use crate::utilities::intrusive_linkedlist::{ListPtr, Intrusive};
use super::*;
use crate::kernel::thread::Thread;
use crate::utilities::NullLock;

const PRIO: u8 = 0;
const STACK_SIZE: usize = 10240;
static mut THREAD_STACK:[u8; STACK_SIZE] = [0; STACK_SIZE];
static THREAD: NullLock<Thread> = NullLock::new(Thread::new());

static mut A :usize = 0;

pub unsafe fn init() {
    THREAD.lock().init("sync2", PRIO, entry, 
                    THREAD_STACK.as_ptr() as usize, STACK_SIZE);
}

pub fn entry(ptr: ListPtr) -> ! {

    let thread = unsafe { Thread::from_ptr_mut(&ptr)};


    {
        thread.sleep(Some(10));
        let num = MTX.lock();
        info!("t2 ocuppy  mutex");
        info!("mutex {}", *num);
        info!("t2 release mutex");
    }

    let mut a = 0;

    loop {

        SEM.get();
        info!("get semaphore");
        a += 1;
        
        if a == 3 {
            thread.sleep(None);
        }

    }
}


