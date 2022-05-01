
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

    match SysCall::from_args(svc_num, r0, r1, r2, r3) {
        Some(syscall) => {
            let ret = match syscall {
                SysCall::Command { command: cmd, arg1 } => {
                    command(cmd, arg1)
                },
                SysCall::HardwareAccess { dev, arg1, arg2} => {
                    hardware_access(dev, arg1, arg2)
                },
            };
            args.offset(0).write_volatile(ret);
        },
        None => {
            panic!("error syscall");
        }
    }
}


fn command(cmd: SysCallCammand, _arg0: usize) -> usize {

    use crate::kernel::sched::scheduler;

    match cmd {
        SysCallCammand::Schedule => unsafe {
            scheduler().schedule();  
        },
    }

    0
}

fn hardware_access(dev: SysCallDevice, arg0: usize, _arg1: usize) -> usize {

    use core::fmt;
    use crate::{board::console, drivers::console::ConsoleDriver};

    match dev {
        SysCallDevice::UartTx => unsafe {
            console().write_fmt(*(arg0 as *const fmt::Arguments)).ok();
        }
        SysCallDevice::UartRx => unsafe {
            if let Some(ch) = console().read() {
                return ch as usize
            }
        }
    }

    0
}

