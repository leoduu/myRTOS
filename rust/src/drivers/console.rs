
use core::fmt;

pub trait ConsoleDriver {

    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

    fn write_char(&self, c: char) -> fmt::Result;

    fn read(&self) -> Option<u8>;
    
    fn flush(&self);
} 


pub fn console() {
    
}
