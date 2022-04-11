
mod shell;
mod test1;
mod test2;

use crate::kernel::ipc::MsgQueue;

static mut MSG_BUF: [usize; 10] = [0; 10];
static mut MSG1: MsgQueue<usize> = unsafe { 
    MsgQueue::new(&mut MSG_BUF, 10)
};

pub unsafe fn user_init() {
    shell::init();
    test1::init();
    test2::init();
}

