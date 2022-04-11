
use core::ops::DerefMut;
use core::cell::RefCell;
use cortex_m::peripheral::{SYST, syst::SystClkSource};
use crate::drivers::timer::software_timer::SoftwareTimerDriver;
use crate::drivers::timer::system_timer::SysTimerDriver;
use crate::kernel::sched::Scheduler;
use cortex_m_rt::exception;
use core::ops::Deref;


pub struct SysTimer {
    timer: RefCell<Option<SYST>>,
    //clocks: CoreClocks,
    irq_handler: fn(), 
    tick: u128,
}

impl SysTimer {
    pub const fn new() -> Self {
        Self {
            timer: RefCell::new(None),
            irq_handler: irq_handler_default,
            tick: 0,
            // clocks: unsafe { core::mem::zeroed() }
        }
    }

    pub fn configure(&mut self, syst: SYST/* , clocks: CoreClocks*/) {
        self.timer = RefCell::new(Some(syst));
        // self.clocks = clocks;
    }

}

impl SysTimerDriver for SysTimer {
    fn init(&self) -> Result<(), &'static str> {
        todo!()
    }

    fn set_irq_handler(&self, _handler: fn()) {
        todo!()
    }

    fn set_periodic(&self, _duration: core::time::Duration) {

        if let Some(timer) = self.timer.borrow_mut().deref_mut() {
            timer.set_clock_source(SystClkSource::External);
            // timer.set_reload(400_000);
            timer.set_reload( 50_000);
            timer.clear_current();
            timer.enable_interrupt();
            timer.enable_counter();
            
        } else {
            panic!("system clock");
        }
    }

    fn irq_handler(&self) -> fn() {
        self.irq_handler
    }

    fn get_count(&self) -> u32 {
        
        if let Some(timer) = self.timer.borrow().deref() {
            timer.cvr.read()
        } else {
            panic!("system clock");
        }
    }

    fn tick(&self) -> u128 {
        self.tick
    }

    fn ticktack(&mut self) {
        self.tick += 1;
    }
}


#[exception]
unsafe fn SysTick() {

    use crate::board::system_timer;
    use crate::board::software_timer;

    cortex_m::interrupt::disable();

    system_timer().ticktack();

    software_timer().check();

    system_timer().irq_handler()();

    cortex_m::interrupt::enable();
}

fn irq_handler_default() {
    unsafe { crate::kernel::sched::scheduler().lock().schedule(); }
}


