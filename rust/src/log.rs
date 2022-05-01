
use core::fmt;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use crate::arch::support::*;
    use crate::kernel::syscall::*;
    use crate::{board::console, drivers::console::ConsoleDriver};

    match cpu_status() {
        CPUStatus::Pmode(_) => {
            console().write_fmt(args).ok();            
        },
        CPUStatus::Umode(SP::MSP) => {
            console().write_fmt(args).ok();
        },
        CPUStatus::Umode(SP::PSP) => unsafe {
            syscall3_0(SysCallClass::HardwareAccess, 
                        SysCallDevice::UartTx as usize, 
                        &args as *const _ as usize);
            
        },
    }
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::log::_print(format_args!($($arg)*)));
}

// /// Prints with a newline.
// ///
// /// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::log::_print(format_args_nl!($($arg)*));
    })
}


/// Prints an info, with a newline.
#[macro_export]
macro_rules! info {
    ($string:expr) => ({
        use core::time::Duration;
        use crate::board::system_timer;
        use crate::drivers::timer::system_timer::SysTimerDriver;

        let timestamp = Duration::from_millis(system_timer().tick() as u64);
        let timestamp_subsec_ms = timestamp.subsec_millis();

        $crate::log::_print(format_args_nl!(
            concat!("[{:>3}.{:03}] ", $string),
            timestamp.as_secs(),
            timestamp_subsec_ms % 1_000,
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        use core::time::Duration;
        use crate::board::system_timer;
        use crate::drivers::timer::system_timer::SysTimerDriver;

        let timestamp = Duration::from_millis(system_timer().tick() as u64);
        let timestamp_subsec_ms = timestamp.subsec_millis();

        $crate::log::_print(format_args_nl!(
            concat!("[{:>3}.{:03}] ", $format_string),
            timestamp.as_secs(),
            timestamp_subsec_ms % 1_000,
            $($arg)*
        ));
    })
}

#[macro_export]
macro_rules! kprintln {
    ($($arg:tt)*) => ({
        use crate::{board::console, drivers::console::ConsoleDriver};
        console().write_fmt(format_args_nl!($($arg)*)).ok();
    })
}

// panic handler
use core::panic::PanicInfo;

#[panic_handler]
fn panic(e: &PanicInfo) -> ! {

    cortex_m::interrupt::disable();

    kprintln!("{:?}", e.location());

    loop {

    }
}
