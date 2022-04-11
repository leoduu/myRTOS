

pub mod timer;
pub mod console;

#[derive(Debug, Eq, PartialEq)]
pub enum DeviceType {
    GPIO,
    Console,
    SysTimer,
    SoftwareTimer,
    // Net,
    // Gpu,
    // Input,
    // Block,
    // Rtc,
    // Intc,
}

pub trait Driver {

    /// Return a compatibility string for identifying the driver.
    fn compatible(&self) -> &'static str; 

    /// Return the type of the driver.
    fn device_type(&self) -> DeviceType;

    /// Called by the kernel to bring up the device.
    ///
    /// # Safety
    ///
    /// - During init, drivers might do stuff with system-wide impact.
    unsafe fn init(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

pub trait DriverManager {
    /// Return a slice of references to all `BSP`-instantiated drivers.
    ///
    /// # Safety
    ///
    /// - The order of devices is the order in which `Driver::init()` is called.
    fn all_device_drivers(&self) -> &[&'static (dyn Driver + Sync)];

    /// Initialization code that runs after driver init.
    ///
    /// For example, device driver code that depends on other drivers already being online.
    fn post_device_driver_init(&self);
}

pub unsafe fn drivers_init() {

    crate::board::board_init();
    // use crate::bsp::board;

    // for i in board::driver_manager()
    //     .all_device_drivers()
    //     .iter()
    // {
    //     if let Err(x) = i.init() {
    //         panic!("Error loading driver: {}: {}", i.compatible(), x);
    //     }
    // }

    // board::driver_manager().post_device_driver_init();
}

pub unsafe fn drivers_list_print() {
    // use crate::bsp::board;
    // use crate::println;

    // for (i, d) in board::driver_manager()
    //     .all_device_drivers()
    //     .iter()
    //     .enumerate() 
    // {            
    //     println!("\t\t[{}] {}", i+1, d.compatible());
    // }    
}
