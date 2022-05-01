
use crate::drivers::{DeviceType, Driver};
use core::time::Duration;


pub trait SysTimerDriver {

    fn init(&self) -> Result<(), &'static str>;

    fn irq_handler(&self) -> fn();

    fn set_irq_handler(&self, handler: fn());

    fn set_periodic(&mut self, duration: Duration);

    fn get_count(&self) -> u32;

    fn tick(&self) -> u128;

    fn ticktack(&mut self);
}

struct SysTimer;

impl Driver for SysTimer {

    fn device_type(&self) -> DeviceType {
        DeviceType::SysTimer
    }

    fn compatible(&self) -> &'static str {
        "System Timer"
    }
}


