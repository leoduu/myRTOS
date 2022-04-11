
pub mod sched;
pub mod thread;
pub mod syscall;
pub mod ipc;
pub mod sync;

use crate::kernel::sched::Scheduler;
use crate::mem;
use crate::drivers;
use crate::user;

#[no_mangle]
pub unsafe fn kernel_init() -> ! {

    cortex_m::interrupt::disable();

    drivers::drivers_init();
    mem::heap_init();

    sched::init();
    user::user_init();

    kernel_main();
}


unsafe fn kernel_main() -> ! {

    // use crate::println;
    // println!("\n\n\n{:?}", mem::allocator());
    // println!("{:?}", *sched::scheduler().lock());

    sched::scheduler().lock().run();
}



