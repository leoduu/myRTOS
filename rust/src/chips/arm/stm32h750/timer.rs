
use crate::drivers::{Driver, DeviceType};
use crate::drivers::timer::basic_timer::BasicTimerDriver;
use core::cell::RefCell;
use core::time::Duration;
use stm32h7::stm32h743v::TIM15;
use super::hal::{
    prelude::*,
    rcc::rec::Tim15,
    rcc::CoreClocks,
    timer::{self, Timer},
};

pub struct BasicTimer {
    timer: RefCell<Timer<TIM15>>,
}

impl BasicTimer {
    pub fn new() -> Self {
        Self {
            timer: unsafe { core::mem::zeroed() }
        }
    }

    pub fn init_timer15(&mut self, timer: TIM15, prec: Tim15, &clocks: &CoreClocks,) {
    
        self.timer = RefCell::new(Timer::tim15(
            timer,
            prec,
            &clocks,
        ))
    }
}

impl BasicTimerDriver for BasicTimer {
    #[inline(always)]
    fn set_irq_handler(&self, _handler: fn()) {
        todo!()
    }

    #[inline(always)]
    fn clear_irq(&self) {
        let mut timer = self.timer.borrow_mut();
        timer.clear_irq();
        timer.reset_counter();
    }

    #[inline(always)]
    fn start(&self, _duration: Duration) {
        let mut timer = self.timer.borrow_mut();
        timer.listen(timer::Event::TimeOut);
        timer.start(1000.ms());
    }

    #[inline(always)]
    fn pause(&self) {
        todo!()
    }

    #[inline(always)]
    fn count(&self) -> u32 {
        todo!()
    }

    #[inline(always)]
    fn set_timeout(&self, duration: Duration) {
        let mut timer = self.timer.borrow_mut();
        timer.set_freq(1.mhz());
        timer.set_timeout(duration);
        timer.listen(timer::Event::TimeOut)
    }
}

impl Driver for BasicTimer {
    #[inline(always)]
    fn compatible(&self) -> &'static str {
        "STM32H750x Timer"
    }

    #[inline(always)]
    fn device_type(&self) -> DeviceType {
        DeviceType::SoftwareTimer
    }
}
