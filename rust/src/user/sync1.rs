
use crate::info;
use crate::utilities::intrusive_linkedlist::{ListPtr, Intrusive};
use super::*;
use crate::kernel::thread::Thread;
use crate::utilities::NullLock;

const PRIO: u8 = 0;
const STACK_SIZE: usize = 1024;
static mut THREAD_STACK:[u8; STACK_SIZE] = [0; STACK_SIZE];
static THREAD: NullLock<Thread> = NullLock::new(Thread::new());

static mut A :usize = 0;

pub unsafe fn init() {
    THREAD.lock().init("sync1", PRIO, entry, 
                    THREAD_STACK.as_ptr() as usize, STACK_SIZE);
}

pub fn entry(ptr: ListPtr) -> ! {

    let thread = unsafe { Thread::from_ptr_mut(&ptr)};

    {
        info!("t1 occupy  mutex");
        let mut num = MTX.lock();
        info!("mutex {}", *num);
        thread.sleep(Some(1000));
        *num = 10;
        info!("t1 release mutex");
    }

    let mut a = 0;
    loop {
        
        if a < 3 {

            SEM.release();

        } else {

            thread.sleep(None);
        }
        
        thread.sleep(Some(1000));
        a += 1;
    }
}



