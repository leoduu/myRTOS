


use cortex_m_rt::exception;
use crate::kernel::syscall::*;

#[exception]
unsafe fn SVCall() {

    use core::arch::asm;
    asm! {
        "
        movs    r0, #1
        msr     primask, r0

        tst     lr, #0x4
        ite     eq
        mrseq   r0, msp
        mrsne   r0, psp

        bl      svc_handler

        mov     r0, #1
        msr     CONTROL, r0

        movs    r0, #0
        msr     primask, r0
        "
    }
}

#[no_mangle]
unsafe fn svc_handler(args: *mut usize) {

    let svc_num = (args.offset(6).read() as *const u8).offset(-2).read();
    let r0 = args.offset(0).read();
    let r1 = args.offset(1).read();
    let r2 = args.offset(2).read();
    let r3 = args.offset(3).read();

    let s = SysCall::from_args(svc_num, r0, r1, r2, r3);

    match s  {
        Some(syscall) => {
            match syscall {
                SysCall::Command { command: cmd, arg1 } => {
                    command(cmd, arg1);
                },
                SysCall::HardwareAccess { dev, arg1, arg2} => {
                    hardware_access(dev, arg1, arg2)
                },
            }
        },
        None => {
            panic!("error syscall");
        }
    }
}


fn command(cmd: SysCallCammand, _arg0: usize) {

    use crate::kernel::sched::{scheduler, Scheduler};

    match cmd {
        SysCallCammand::Schedule => unsafe {
            scheduler().lock().schedule();  
        },
    }

}

fn hardware_access(dev: SysCallDevice, arg0: usize, _arg1: usize) {

    use core::fmt;
    use crate::{board::console, drivers::console::ConsoleDriver};

    match dev {
        SysCallDevice::Uart => unsafe {
            console().write_fmt(*(arg0 as *const fmt::Arguments)).ok();
        }
    }

}

