
use crate::println;
use super::*;
use crate::kernel::thread::Thread;
use crate::utilities::NullLock;

const STACK_SIZE: usize = 512;
static mut THREAD_STACK:[u8; STACK_SIZE] = [0; STACK_SIZE];
static THREAD: NullLock<Thread> = NullLock::new(Thread::new());

static mut A :usize = 0;

pub unsafe fn init() {
    THREAD.lock().init("test1", entry, THREAD_STACK.as_ptr() as usize, STACK_SIZE);
}

pub fn entry() -> ! {

    println!("t1 {:?}", crate::arch::support::cpu_status());

    loop {
        // THREAD.lock().timer(1000, timer_handler);
        THREAD.lock().sleep(500);
        
        unsafe { 
            if MSG1.tramsmit(A).is_ok() {
                A += 1;
            }
        };
    }
}


pub fn timer_handler() {

    unsafe {
        use super::*;
        if MSG1.tramsmit(A).is_ok() {
            A += 1;
        }
    }

    // println!("+++");
}

