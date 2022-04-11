
use core::fmt;

pub struct MsgQueue<'a, T> {

    buf: &'a mut [T],
    size: u16,

    curr: u16,
    stock: u16,
}

impl<'a, T> MsgQueue<'a, T> where T: Copy + fmt::Debug {

    pub const fn new(buf: &'a mut [T], size: u16) -> Self {
        Self {
            buf,
            size,
            curr: 0,
            stock: 0
        }
    }
    
    pub fn tramsmit(&mut self, data: T) -> Result<(), ()> {

        if self.stock >= self.size {
            return Err(())
        }

        let mut loc = self.curr + self.stock;

        if loc >= self.size {
            loc -= self.size;
        }

        // crate::println!("loc {}", loc);
        
        self.buf[loc as usize] = data;
        self.stock += 1;

        // crate::println!("TT {:?}", self);

        Ok(())
    }

    pub fn receive(&mut self) -> Option<T> {

        if self.stock == 0 {
            return None
        }

        let data = self.buf[self.curr as usize];

        self.stock -= 1;
        self.curr  += 1;
        if self.curr >= self.size {
            self.curr -= self.size
        }

        // crate::println!("RR {:?}", self);

        Some(data)
    }
}

impl<'a, T> fmt::Debug for MsgQueue<'a, T> where T: Copy + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        writeln!(f, "{:?}", self.buf)?;
        writeln!(f, "size[{}] stock[{}] curr[{}]", self.size, self.stock, self.curr)
    }
}
