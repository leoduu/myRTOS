

use embedded_hal::prelude::*;
use stm32h7xx_hal::serial::{Tx, Rx};

use crate::drivers::{Driver, DeviceType};
use crate::drivers::console::*;
use core::fmt::{self, Write};
use stm32h7::stm32h743v::USART1;
use stm32h7::stm32h743v::DMA1;
use super::hal::{
    rcc::rec::Usart1,
    rcc::rec::Dma1,
    rcc::CoreClocks,
    serial::config::Config,
    serial::Serial,
    dma::dma::StreamsTuple,
    dma::dma::DmaConfig,
    dma::MemoryToPeripheral, 
    dma::Transfer,
    dma::dma::Stream0,
};


pub struct Console {
    tx: Option<Tx<USART1>>,
    rx: Option<Rx<USART1>>,
    stream: Option<Stream0<DMA1>>,
}

impl Console {
    pub const fn new() -> Self {
        Self {
            tx: None,
            rx: None,
            stream: None,
        }
    }

    pub fn init_usart1(&mut self, 
                        usart: USART1, 
                        config: Config, 
                        prec: Usart1, 
                        &clocks: &CoreClocks, 
                        sync: bool,
                        dma: DMA1,
                        dma1: Dma1) {
    
        let uart1 = Serial::usart1(
            usart,
            config,
            prec,
            &clocks,
            sync,
        ).unwrap();

        let (tx, rx) = uart1.split();

        self.tx = Some(tx);
        self.rx = Some(rx);

        self.stream = Some(StreamsTuple::new(dma, dma1).0);

    }

    pub fn write_dma(&mut self, args: fmt::Arguments) {

        let s = unsafe { &mut *(args.as_str().unwrap() as *const _ as *mut [u8]) };


        // let buffer: &'static mut [u8; 10] = {
        //     let buf: &mut [MaybeUninit<u8>; 10] =
        //         unsafe { mem::transmute(&mut SHORT_BUFFER) };
    
        //     for (i, value) in buf.iter_mut().enumerate() {
        //         unsafe {
        //             value.as_mut_ptr().write(i as u8 + 96); // 0x60, 0x61, 0x62...
        //         }
        //     }
        //     unsafe { mem::transmute(buf) }
        // };

        let config = DmaConfig::default().memory_increment(true);
        let tx = self.tx.take().unwrap();
        let stream = self.stream.take().unwrap();

        let mut transfer: Transfer<_, _, MemoryToPeripheral, _, _> =
            Transfer::init(stream, tx, s, None, config);
        
        transfer.start(|serial| {
            // This closure runs right after enabling the stream
    
            // Enable DMA Tx buffer by setting the DMAT bit in the USART_CR1
            // register
            serial.enable_dma_tx();
        });
    
        while !transfer.get_transfer_complete_flag() {}
        let (stream, tx, _, _) = transfer.free();

        self.tx = Some(tx);
        self.stream = Some(stream);
    }
}

impl ConsoleDriver for Console {
    #[inline(always)]
    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        if let Some(tx) = &mut self.tx {
            tx.write_fmt(args)
        } else {
            panic!()
        }
    }

    #[inline(always)]
    fn write_char(&mut self, c: char) -> fmt::Result {
        if let Some(tx) = &mut self.tx {
            tx.write_char(c)
        } else {
            panic!()
        }
    }

    #[inline(always)]
    fn read(&mut self) -> Option<u8> {
        if let Some(rx) = &mut self.rx {
            rx.read().ok()
        } else {
            Some(0)
        }
    }

    #[inline(always)]
    fn flush(&mut self) {
        if let Some(tx) = &mut self.tx {
            tx.flush().ok();
        }
    }
}

impl Driver for Console {
    #[inline(always)]
    fn compatible(&self) -> &'static str {
        "STM32H750x Console (Uart1)"
    }

    #[inline(always)]
    fn device_type(&self) -> DeviceType {
        DeviceType::Console
    }
}
