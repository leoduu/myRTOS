
use core::time::Duration;


pub trait BasicTimerDriver {
    
    fn set_timeout(&self, duration: Duration);

    fn set_irq_handler(&self, handler: fn());

    fn clear_irq(&self);

    fn start(&self, duration: Duration);

    fn pause(&self);

    fn count(&self) -> u32;
}


