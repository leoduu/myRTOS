
use super::system_timer::SysTimerDriver;
use crate::drivers::{Driver, DeviceType};
use crate::utilities::intrusive_linkedlist::*;
use crate::{container_of_mut, container_of};
use crate::board::{software_timer, system_timer};
use crate::kernel::thread::Thread;
use core::fmt;

pub trait SoftwareTimerDriver {
    
    fn add(&self, timer: &mut Timer);

    unsafe fn check(&self);
}

pub struct SoftWareTimer{
    list: LinkedList,
}

impl SoftWareTimer {
    pub const fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }
}

impl SoftwareTimerDriver for SoftWareTimer {
    fn add(&self, timer: &mut Timer) {

        unsafe {
            let timer_ptr = ListPtr::new_uncheck(&mut timer.node);

            let mut ptr = self.list.head().unwrap();

            while let Some(p) = ptr.next() {

                let t = Timer::from_ptr(&p);
                if t.timeout > timer.timeout {
                    // insert to prev
                    timer_ptr.set_prev(p.prev());
                    timer_ptr.set_next(Some(p));
                    p.prev().unwrap().set_next(Some(timer_ptr));
                    p.set_prev(Some(timer_ptr));
                    return;
                }
                ptr = ptr.next().unwrap();
            }

            ptr.insert(timer_ptr);
        }

        // crate::println!("a {:?}", self);
    }

    unsafe fn check(&self) {

        let curr_tick = system_timer().tick();

        // skip first node
        let mut ptr = self.list.head().unwrap().next();

        while let Some(p) = ptr {

            let timer = Timer::from_ptr_mut(&p);
            if timer.timeout <= curr_tick {

                self.list.detach(p);
                match timer.mode {
                    TimerMode::Block => {                        
                        let thread = Thread::from_timer(timer);
                        thread.wakup();
                    },
                    TimerMode::NonBlock(f) => f(),
                }
                timer.status = TimerStatus::Leisure;
            }
            ptr = p.next();
        }

    }
}

impl Driver for SoftWareTimer {
    fn compatible(&self) -> &'static str {
        "Software Timer"
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::SoftwareTimer
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        
        self.list.push_front(TIMER_HEAD.node_ptr().unwrap());

        Ok(())
    }
}

impl fmt::Debug for SoftWareTimer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        writeln!(f, ">>> software timer (tick: {})", system_timer().tick())?;
        let mut cnt = 0;
        unsafe {
            let mut ptr = self.list.head().unwrap().next();
            while let Some(p) = ptr {
                writeln!(f, "{:?}", Timer::from_ptr(&p))?;
                cnt += 1;
                ptr = p.next()
            }
        }
        write!(f, "{} timers in list", cnt)
    }
}


static mut TIMER_HEAD: Timer = Timer::new();

#[derive(PartialEq)]
pub enum TimerMode {
    Block,
    NonBlock(fn()),
}

impl fmt::Debug for TimerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block => write!(f, "Block"),
            Self::NonBlock(handler) => write!(f, "NonBlock({:?})", handler as *const _),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TimerStatus {
    Leisure,
    Running,
    Stop,
}

pub struct Timer {
    pub node: ListNode,
    pub stop_tick: u128,
    pub timeout:  u128,
    pub mode: TimerMode,
    pub status: TimerStatus,
}

impl Timer {
    pub const fn new() -> Self {
        Self {
            node: ListNode::new(),
            stop_tick: 0,
            timeout: 0,
            mode: TimerMode::Block,
            status: TimerStatus::Leisure,
        }
    }

    pub fn timeout(&mut self, tick: u128, mode: TimerMode) {

        if self.status != TimerStatus::Leisure {
            return;
        }

        self.timeout = tick + system_timer().tick();
        self.mode = mode;
        self.status = TimerStatus::Running;

        software_timer().add(self);
    }

    fn stop(&mut self) {
        self.status = TimerStatus::Stop;
    }

    pub fn node_ptr(&mut self) -> Option<ListPtr> {
        ListPtr::new(&mut self.node)
    }
}

impl Intrusive for Timer {
    type Item = Timer;

    unsafe fn from_ptr(ptr: &ListPtr) -> &Self::Item {
        container_of!(ptr.0.as_ref(), Timer, node)
    }

    unsafe fn from_ptr_mut(ptr: &ListPtr) -> &mut Self::Item {
        container_of_mut!(ptr.0.as_ref(), Timer, node)
    }
}


impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let thread = unsafe { Thread::from_timer(&self) };

        writeln!(f, "timer ({:?})", self as *const _)?;
        writeln!(f, "\towner  | {}", thread)?;
        writeln!(f, "\ttimeout| {}", self.timeout)?;
        writeln!(f, "\tmode   | {:?}", self.mode)?;
        writeln!(f, "\tstatus | {:?}", self.status)?;
        write!(f, "\t{:?}", self.node)
    }
}

