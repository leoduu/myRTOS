
use core::fmt;
use heapless::Deque;

pub struct MsgQueue<const N: usize> {
    len: Deque<u16, N>,
    buf: [u8; N],
    curr: u16,
    stock: u16,
}

impl<const N: usize> MsgQueue<N> {

    pub const fn new() -> Self {
        Self {
            len: Deque::new(),
            buf: [0; N],
            curr: 0,
            stock: 0
        }
    }
    
    pub fn transmit(&mut self, data: &mut [u8], len: u16) -> Result<(), ()> {

        if self.stock + len > N as u16 {
            return Err(())
        }

        let mut loc = self.curr + self.stock;
        
        for i in 0..len {
            self.buf[loc as usize] =  core::mem::take(&mut data[i as usize]);
            loc += 1;
            if loc >= N as u16 {
                loc = 0;
            }
        }
        
        self.stock += len;
        self.len.push_back(len).ok();

        Ok(())
    }

    pub fn receive(&mut self, buf: &mut [u8]) -> Option<u16> {

        if let Some(len) = self.len.pop_front() {
            let mut loc = self.curr;
            for i in 0..len {
                buf[i as usize] = core::mem::take(&mut self.buf[loc as usize]);
                loc += 1;
                if loc >= N as u16 {
                    loc = 0;
                }
            }

            self.stock -= len;
            self.curr = loc;
            Some(len)
        } else {
            None
        }
    }
}

impl<const N: usize> fmt::Debug for MsgQueue<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        writeln!(f, "{:?}", self.buf)?;
        writeln!(f, "msg[{}] size[{}] stock[{}] curr[{}]", 
                self.len.len(), N, self.stock, self.curr)
    }
}
