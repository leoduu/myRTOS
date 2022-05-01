pub mod timer;
pub mod context;
pub mod mpu;
pub mod syscall;
pub mod support;
mod boot;

use cortex_m_rt::exception;

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {

    use crate::kprintln;

    cortex_m::interrupt::disable();

    kprintln!("HardFault");
    kprintln!("\tr0 :{:#010x}  r1 :{:#010x}", ef.r0, ef.r1);
    kprintln!("\tr2 :{:#010x}  r3 :{:#010x}", ef.r2, ef.r3);
    kprintln!("\tr12:{:#010x}  lr :{:#010x}", ef.r12, ef.lr);
    kprintln!("\tpc :{:#010x}  psr:{:#010x}", ef.pc, ef.xpsr);
    
    loop{}
}

