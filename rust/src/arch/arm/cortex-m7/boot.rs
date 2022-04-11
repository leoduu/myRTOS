
use cortex_m_rt::entry;

#[entry]
fn boot() -> ! {

    unsafe {
        crate::kernel::kernel_init();
    }
}
