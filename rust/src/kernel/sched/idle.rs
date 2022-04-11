
use crate::kernel::thread::Thread;
use crate::utilities::NullLock;

const STACK_SIZE: usize = 256;
static mut IDLE_STACK:[u8; STACK_SIZE] = [0; STACK_SIZE];
static IDLE: NullLock<Thread> = NullLock::new(Thread::new());


pub unsafe fn init() {

    IDLE.lock().init("idle", entry, IDLE_STACK.as_ptr() as usize, STACK_SIZE);

}

fn entry() -> ! {

    loop {

    }
}

