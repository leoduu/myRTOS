
use core::fmt;

pub trait ConsoleDriver {

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result;

    fn write_char(&mut self, c: char) -> fmt::Result;

    fn read(&mut self) -> Option<u8>;
    
    fn flush(&mut self);
} 

