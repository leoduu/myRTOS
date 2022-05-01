
mod shell;
mod ipc1;
mod ipc2;
mod sync1;
mod sync2;

use crate::kernel::ipc::*;
use crate::kernel::sync::*;
use crate::utilities::NullLock;

const MBOX_SIZE: usize = 3;
lazy_static!{
    static ref MBOX: NullLock<MailBox<usize, MBOX_SIZE>> = NullLock::new(MailBox::new());
}
const MQUEUE_SIZE: usize = 10;
static MQUEUE: NullLock<MsgQueue<MQUEUE_SIZE>> = NullLock::new(MsgQueue::new());

static CNT: usize = 0;
static MTX: Mutex<usize> = Mutex::new(CNT);
static SEM: Semaphore = Semaphore::new(0);

pub unsafe fn user_init() {
    shell::init();
    // ipc1::init();
    // ipc2::init();
    // sync1::init();
    // sync2::init();
}

