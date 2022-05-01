

use core::fmt;
use cortex_m_rt::exception;
use crate::utilities::intrusive_linkedlist::ListPtr;

#[no_mangle]
pub static mut CURR_SP: usize = 0;
#[no_mangle]
pub static mut NEXT_SP: usize = 0;
#[no_mangle]
pub static mut INT_FLAG: usize = 0;

#[repr(C)]
pub struct StackFrame {
    // mamual save
    // r4 - r11
    regs_4_11: [usize; 8],
    // auto save
    // r0, r1, r2, r3
    // r12, lr, pc, xpsr
    regs_auto: [usize; 8],
}

impl StackFrame {
    pub fn new(ptr: ListPtr, entry: usize) -> Self {
        let mut stackframe = Self {
            regs_4_11: [0; 8],
            regs_auto: [0; 8],
        };
        stackframe.regs_auto[0] = unsafe {core::mem::transmute(ptr)};
        stackframe.regs_auto[6] = entry;
        stackframe.regs_auto[7] = 0x0100_0000;
        stackframe
    }
}

impl fmt::Debug for StackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "addr {:?}", self as *const _).ok();
        writeln!(f, " r4  {:#010x}, r5  {:#010x}", self.regs_4_11[0], self.regs_4_11[1]).ok();
        writeln!(f, " r6  {:#010x}, r7  {:#010x}", self.regs_4_11[2], self.regs_4_11[3]).ok();
        writeln!(f, " r8  {:#010x}, r9  {:#010x}", self.regs_4_11[4], self.regs_4_11[5]).ok();
        writeln!(f, " r10 {:#010x}, r11 {:#010x}", self.regs_4_11[6], self.regs_4_11[6]).ok();
        writeln!(f, " r0  {:#010x}, r1  {:#010x}", self.regs_auto[0], self.regs_auto[1]).ok();
        writeln!(f, " r2  {:#010x}, r3  {:#010x}", self.regs_auto[2], self.regs_auto[3]).ok();
        writeln!(f, " r12 {:#010x}, lr  {:#010x}", self.regs_auto[4], self.regs_auto[5]).ok();
        writeln!(f, " pc  {:#010x}, psr {:#010x}", self.regs_auto[6], self.regs_auto[7]).ok();
        Ok(())
    }
}

pub unsafe fn context_switch_interrupt(next_sp: *mut usize) {

    NEXT_SP = next_sp as usize;
    INT_FLAG = 0;

    cortex_m::peripheral::SCB::set_pendsv();
}

pub unsafe fn context_switch(curr_sp: *mut usize, next_sp: *mut usize) {
    

    CURR_SP = curr_sp as usize;
    NEXT_SP = next_sp as usize;
    INT_FLAG = 1;

    cortex_m::peripheral::SCB::set_pendsv();
}


#[exception]
unsafe fn PendSV() {
    use core::arch::asm;

    asm!{"
        // auto generate
        // push	{{r7, lr}}
        // mov 	r7, sp
        pop     {{r7, lr}}

        // disable interrupt
        movs    r0, #1
        msr     primask, r0
        
        // check interrupt flag
        ldr     r0, =INT_FLAG
        ldr     r0, [r0]
        cbz     r0, 1f

        // store regs info to current sp
        mrs     r0, psp
        stmdb   r0!, {{r4 - r11}}
        ldr     r1, =CURR_SP
        ldr     r1, [r1]
        str     r0, [r1]
        
        // load regs form next sp
    1:
        ldr     r2, =NEXT_SP
        ldr     r2, [r2]
        ldr     r2, [r2]
        ldmia   r2!, {{r4 - r11}}
        msr     psp, r2 

        // enable insterrutp
        movs    r0, #0
        msr     primask, r0

        isb

        // switch to unprivileged
        mov     r0, #1
        msr     CONTROL, r0

        // return 
        orr     lr, lr, #0x04
        bx      lr
    "};
}

