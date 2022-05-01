
use core::fmt;
use core::cell::Cell;
use crate::kernel::thread::Thread;
use crate::utilities::intrusive_linkedlist::{ListPtr, Intrusive};

pub struct MailBox<T, const N: usize> {
    buf: [T; N],
    current: u16,
    stock: u16,
    receive: Cell<Option<ListPtr>>
}

impl<T, const N: usize> MailBox<T, N> where T: Default {

    pub fn new() -> Self {
    
        Self {
            buf: unsafe {core::mem::zeroed()},
            current: 0,
            stock: 0,
            receive: Cell::new(None),
        }
    }
    
    pub fn transmit(&mut self, data: T) -> Result<(), ()> {

        if self.stock >= N as u16 {
            return Err(())
        }

        let mut loc = self.current + self.stock;

        if loc >= N as u16 {
            loc -= N as u16;
        }
        
        self.buf[loc as usize] = data;
        self.stock += 1;

        if let Some(ptr) = self.receive.get() {
            let thread = unsafe { Thread::from_ptr_mut(&ptr) };
            thread.wakup();
        }

        Ok(())
    }

    pub fn receive(&mut self) -> Option<T> {

        if self.stock == 0 {
            return None
        }

        let data = core::mem::take(&mut self.buf[self.current as usize]);

        self.stock -= 1;
        self.current  += 1;
        if self.current >= N as u16 {
            self.current -= N as u16
        }

        Some(data)
    }
}

impl<T, const N: usize> fmt::Debug for MailBox<T, N> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        writeln!(f, "{:?}", self.buf)?;
        writeln!(f, "size[{}] stock[{}] curr[{}]", N, self.stock, self.current)
    }
}
