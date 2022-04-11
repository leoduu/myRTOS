
use core::mem::size_of;
use core::fmt;
use crate::drivers::timer::software_timer::Timer;
use crate::arch::context::StackFrame;
use crate::kernel::sched::{scheduler, Scheduler};
use crate::{container_of, container_of_mut};
use crate::utilities::intrusive_linkedlist::*;
use crate::drivers::timer::software_timer::*;

const THREAD_NAME_LEN: usize = 10;
type ThreadName = [u8; THREAD_NAME_LEN];

#[derive(PartialEq, Debug)]
pub enum ThreadStatus {
    Ready,
    Sleep,
    Terminal
}

impl fmt::Display for ThreadStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ready => write!(f, "Ready"),
            Self::Sleep => write!(f, "Sleep"),
            Self::Terminal => write!(f, "Terminal"),
        }
    }
}

pub struct Thread {

    node: ListNode,

    pub name: ThreadName,           // Thread Name
    //pid: u32,                     // Thread ID
    
    pub status: ThreadStatus,       // Thread status

    sp: usize,
    stack_start: usize,         // Stack start address    
    stack_size: usize,          // Stack Size

    timer: Timer,    
}


impl Thread {

    pub const fn new() -> Self {
        Self {
            name: [0; 10],
            node: ListNode::new(),
            status: ThreadStatus::Ready,
            sp: 0,
            stack_size: 0,
            stack_start: 0,
            timer: Timer::new(),
        }
    }

    
    pub unsafe fn init(&mut self, 
                        name: &str,
                        entry: fn() -> !,
                        stack_start: usize,
                        stack_size: usize) 
    {
        for i in 0..name.len() {
            self.name[i] = name.as_bytes()[i];
        }
        self.stack_start = stack_start;
        self.stack_size = stack_size;
        self.sp = stack_start + stack_size - size_of::<StackFrame>();

        unsafe {
            (self.sp as *mut StackFrame).write_volatile(StackFrame::new(entry as usize));
        }

        scheduler().lock().push(ListPtr::new(&mut self.node).unwrap())
    }

    pub fn sleep(&mut self, tick: u128) {

        self.status = ThreadStatus::Sleep;
        self.timer.timeout(tick, TimerMode::Block);

    }

    pub fn timer(&mut self, tick: u128, handler: fn()) {

        self.timer.timeout(tick, TimerMode::NonBlock(handler))
    }

    #[inline]
    pub fn show_name(&self) {
        // for i in 0..THREAD_NAME_LEN {
        //     crate::print!("{}", self.name[i] as char);
        // }
    }

    #[inline]
    pub fn sp(&mut self) -> *mut usize {
        &self.sp as *const usize as *mut usize
    }

    pub unsafe fn from_timer(timer: &Timer) -> &mut Self {
        container_of_mut!(timer, Thread, timer)
    }
}


impl Intrusive for Thread {
    type Item = Thread;

    #[inline]
    unsafe fn from_ptr(ptr: &ListPtr) -> &Self::Item {
        container_of!(ptr.0.as_ref(), Thread, node)
    }

    #[inline]
    unsafe fn from_ptr_mut(ptr: &ListPtr) -> &mut Self::Item {
        container_of_mut!(ptr.0.as_ref(), Thread, node)
    }
}

impl fmt::Debug for Thread {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        for i in 0..THREAD_NAME_LEN {
            write!(f, "{}", self.name[i] as char)?;
        }
        writeln!(f, "({:?})", self as *const Self)?;
        writeln!(f, "\tstate| {:?}", self.status)?;
        writeln!(f, "\tsp   | {:#010x}", self.sp)?;
        writeln!(f, "\tstack| {:#010x} - {:#010x}", self.stack_start, self.stack_start+self.stack_size)?;
        writeln!(f, "\tsize | {} Byte", self.stack_size)?;
        writeln!(f, "\tused | {} Byte", self.stack_start+self.stack_size-self.sp)?;
        writeln!(f, "\tnode | {:?}", self.node)?;
        Ok(())
    }
}



