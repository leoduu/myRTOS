
use core::cell::RefCell;

pub struct Semaphore {
    data: RefCell<u8>,
}

unsafe impl Sync for Semaphore {}
unsafe impl Send for Semaphore {}

impl Semaphore {    
    pub const fn new(data: u8) -> Self {
        Self {
            data: RefCell::new(data)
        }
    }

    pub fn get(&self) {

        let data = self.data.as_ptr();

        unsafe {
            while data.read_volatile() == 0 {}
            data.write_volatile(*data-1);
        }

    }

    pub fn try_get(&self) -> Result<(), ()>  {

        let data = self.data.as_ptr();

        unsafe {
            if data.read_volatile() > 0 {
                data.write_volatile(*data-1);
                Ok(())
            } else {
                Err(())
            }
        }
    }

    pub fn release(&self) {

        let data = self.data.as_ptr();

        unsafe {
            data.write_volatile(*data+1);
        }
    }

    pub fn data(&self) -> u8 {
        *self.data.borrow()
    }
}
