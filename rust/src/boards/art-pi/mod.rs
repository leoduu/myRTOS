
#[path = "../../chips/arm/stm32h750/mod.rs"]
mod stm;

use stm::console::Console;
use stm::timer::BasicTimer;

use crate::drivers::Driver;
use crate::drivers::timer::system_timer::SysTimerDriver;
use crate::drivers::timer::software_timer::{SoftWareTimer, SoftwareTimerDriver};
use crate::utilities::NullLock;

use crate::drivers::console::ConsoleDriver;
use crate::drivers::timer::basic_timer::BasicTimerDriver;
use crate::arch::timer::SysTimer;
use crate::arch::mpu::MPUControl;

use core::time::Duration;
use stm32h7xx_hal::{interrupt, pac};

// static peripheral
static mut CONSOLE: Console = Console::new();

static mut SYSTEM_TIMER: SysTimer = SysTimer::new();

static mut SOFTWARE_TIMER: SoftWareTimer = SoftWareTimer::new();

static mut MPU: MPUControl = MPUControl::new();

// static peripheral
lazy_static!{    
    static ref BOARD_TIMER: NullLock<BasicTimer> = 
        NullLock::new(BasicTimer::new());
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
pub fn console() -> &'static mut impl ConsoleDriver {
    unsafe { &mut CONSOLE }
}

pub fn system_timer() -> &'static mut impl SysTimerDriver {
    unsafe { &mut SYSTEM_TIMER }
}

pub fn software_timer() -> &'static mut impl SoftwareTimerDriver {
    unsafe { &mut SOFTWARE_TIMER }
}

#[inline(never)]
// init 
pub unsafe fn board_init() {

    use stm32h7xx_hal::{
        pwr::PwrExt,
        serial,
        prelude::*,
        rcc::RccExt, 
    };

    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Configure clock tree with HSI and PLL to reach 200Mhz
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();
    let rcc = dp.RCC.constrain();
    let ccdr = rcc
        .use_hse(25.mhz())
        .sys_ck(400.mhz())
        .hclk(200.mhz())
        .freeze(pwrcfg, &dp.SYSCFG);

    
    // Acquire the GPIOA peripheral
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);

    // initialize serial
    gpioa.pa9.into_alternate_af7();
    gpioa.pa10.into_alternate_af7();

    CONSOLE.init_usart1(
        dp.USART1,
        serial::config::Config::default().baudrate(115200.bps()),
        ccdr.peripheral.USART1,
        &ccdr.clocks,
        false,
        dp.DMA1,
        ccdr.peripheral.DMA1,
    ); 

    // BOARD_TIMER.lock().init_timer15(dp.TIM15, ccdr.peripheral.TIM15, &ccdr.clocks);
    // BOARD_TIMER.lock().start(Duration::from_millis(1000));

    SYSTEM_TIMER.configure(cp.SYST);
    SYSTEM_TIMER.set_periodic(Duration::from_micros(1000));

    SOFTWARE_TIMER.init().ok();

    // println!("h {}", ccdr.clocks.hclk().0);
    // println!("c {}", ccdr.clocks.c_ck().0);

    MPU.init(cp.MPU, &mut cp.SCB);

    // cp.NVIC.set_priority(interrupt::TIM15, 1);
    // cp.NVIC.set_priority(interrupt::DMA1_STR0, 1);
    // NVIC::unmask(interrupt::TIM15);
    // NVIC::unmask(interrupt::DMA1_STR0);
}


#[interrupt]
fn DMA1_STR0() {
    // crate::println!("DMA1");
}

#[interrupt]
fn TIM15() {
    // println!("borad timer");
    BOARD_TIMER.lock().clear_irq();
}

