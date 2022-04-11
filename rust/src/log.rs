
use core::fmt;
use crate::{board::console, drivers::console::ConsoleDriver};

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use crate::arch::support::*;
    use crate::kernel::syscall::*;


    match cpu_status() {
        CPUStatus::Pmode(_) => {
            console().write_fmt(args).ok();            
        },
        CPUStatus::Umode(SP::MSP) => {
            console().write_fmt(args).ok();
        },
        CPUStatus::Umode(SP::PSP) => unsafe {
            syscall3_0(SysCallClass::HardwareAccess, 
                        SysCallDevice::Uart as usize, 
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


// /// Prints an info, with a newline.
// #[macro_export]
// macro_rules! info {
//     ($string:expr) => ({
//         use crate::drivers::timer::{system_timer, system_timer::Interface};

//         let timestamp = system_timer().get_cycle();
//         let timestamp_subsec_us = timestamp.subsec_micros();

//         $crate::log::_print(format_args_nl!(
//             concat!("[  {:>3}.{:06}] ", $string),
//             timestamp.as_secs(),
//             timestamp_subsec_us % 1_000_000,
//         ));
//     });
//     ($format_string:expr, $($arg:tt)*) => ({
//         use crate::drivers::timer::{system_timer, system_timer::Interface};

//         let timestamp = system_timer().get_cycle();
//         let timestamp_subsec_us = timestamp.subsec_micros();

//         $crate::log::_print(format_args_nl!(
//             concat!("[  {:>3}.{:06}] ", $format_string),
//             timestamp.as_secs(),
//             timestamp_subsec_us % 1_000_000,
//             $($arg)*
//         ));
//     })
// }

// /// Prints a warning, with a newline.
// #[macro_export]
// macro_rules! warn {
//     ($string:expr) => ({
//         use crate::drivers::timer::{system_timer, system_timer::Interface};

//         let timestamp = system_timer().get_cycle();
//         let timestamp_subsec_us = timestamp.subsec_micros();

//         $crate::log::_print(format_args_nl!(
//             concat!("[W {:>3}.{:03}{:03}] ", $string),
//             timestamp.as_secs(),
//             timestamp_subsec_us / 1_000,
//             timestamp_subsec_us % 1_000
//         ));
//     });
//     ($format_string:expr, $($arg:tt)*) => ({
//         use crate::drivers::timer::system_timer;

//         let timestamp = system_timer().get_cycle();
//         let timestamp_subsec_us = timestamp.subsec_micros();

//         $crate::log::_print(format_args_nl!(
//             concat!("[W {:>3}.{:03}{:03}] ", $format_string),
//             timestamp.as_secs(),
//             timestamp_subsec_us / 1_000,
//             timestamp_subsec_us % 1_000,
//             $($arg)*
//         ));
//     })
// }


macro_rules! kprintln {
    ($($arg:tt)*) => ({
        console().write_fmt(format_args_nl!($($arg)*)).ok();
    })
}

// panic handler
use core::panic::PanicInfo;

#[panic_handler]
fn panic(e: &PanicInfo) -> ! {

    cortex_m::interrupt::disable();

    if cfg!(feature = "logs") {
        kprintln!("{:?}", e.location());
    }

    loop {

    }
}
