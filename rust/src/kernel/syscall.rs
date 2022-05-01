use core::convert::TryFrom;
use core::arch::asm;

pub fn syscall3_0(class: SysCallClass, arg0: usize, arg1: usize) {

    match class {
        SysCallClass::Command => unsafe {
            asm!{ 
                "svc #1",
                in("r0") arg0,
                in("r1") arg1,
            }
        },
        SysCallClass::HardwareAccess => unsafe {
            asm!{ 
                "svc #2",
                in("r0") arg0,
                in("r1") arg1,
            }
        },
    }
}

pub fn syscall2_0(class: SysCallClass, arg0: usize) {
    match class {
        SysCallClass::Command => unsafe {
            asm!{ 
                "svc #1",
                in("r0") arg0,
            }
        },
        SysCallClass::HardwareAccess => unsafe {
            asm!{ 
                "svc #2",
                in("r0") arg0,
            }
        },
    }
}

pub fn syscall2_1(class: SysCallClass, mut arg0: usize) -> usize {
    match class {
        SysCallClass::Command => unsafe {
            asm!{ 
                "svc #1",
                inout("r0") arg0,
            };
            arg0
        },
        SysCallClass::HardwareAccess => unsafe {
            asm!{ 
                "svc #2",
                inout("r0") arg0,
            };
            arg0
        },
    }
}



#[derive(Debug, Eq, PartialEq)]
pub enum SysCall {
    Command {
        command: SysCallCammand,
        arg1: usize,
    },

    HardwareAccess {
        dev: SysCallDevice,
        arg1: usize,
        arg2: usize,
    },
}

impl SysCall {
    pub fn from_args(
        syscall_number: u8,
        r0: usize,
        r1: usize,
        r2: usize,
        _r3: usize,
    ) -> Option<SysCall> {

        match SysCallClass::try_from(syscall_number) {

            Ok(SysCallClass::Command) => Some(SysCall::Command {
                command: SysCallCammand::try_from(r0).unwrap(),
                arg1: r1,
            }),

            Ok(SysCallClass::HardwareAccess) => Some(SysCall::HardwareAccess{ 
                dev: SysCallDevice::try_from(r0).unwrap(), 
                arg1: r1, 
                arg2: r2,
            }),

            Err(_) => None,
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum SysCallClass {
    Command = 1,
    HardwareAccess = 2,
}

impl TryFrom<u8> for SysCallClass {
    type Error = u8;
    
    fn try_from(num: u8) -> Result<SysCallClass, u8> {
        match num {
            1 => Ok(SysCallClass::Command),
            2 => Ok(SysCallClass::HardwareAccess),
            other => Err(other)
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum SysCallCammand {
    Schedule = 1,
}

impl TryFrom<usize> for SysCallCammand {
    type Error = usize;
    
    fn try_from(num: usize) -> Result<SysCallCammand, usize> {
        match num {
            1 => Ok(SysCallCammand::Schedule),
            other => Err(other)
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum SysCallDevice {
    UartTx = 1,
    UartRx = 2,
}

impl TryFrom<usize> for SysCallDevice {
    type Error = usize;
    
    fn try_from(num: usize) -> Result<SysCallDevice, usize> {
        match num {
            1 => Ok(SysCallDevice::UartTx),
            2 => Ok(SysCallDevice::UartRx),
            other => Err(other)
        }
    }
}




