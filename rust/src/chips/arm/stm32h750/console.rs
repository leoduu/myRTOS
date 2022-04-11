

use embedded_hal::prelude::*;
use stm32h7xx_hal::serial::{Tx, Rx};

use crate::drivers::{Driver, DeviceType};
use crate::drivers::console::*;
use core::cell::RefCell;
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
    // pub uart: RefCell<Option<Serial<USART1>>>,
    tx: RefCell<Option<Tx<USART1>>>,
    rx: RefCell<Option<Rx<USART1>>>,
    stream: RefCell<Option<Stream0<DMA1>>>,
}

impl Console {
    pub const fn new() -> Self {
        Self {
            // uart: RefCell::new(None)
            tx: RefCell::new(None),
            rx: RefCell::new(None),
            stream: RefCell::new(None),
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

        self.tx.replace(Some(tx));
        self.rx.replace(Some(rx));

        self.stream.replace(Some(StreamsTuple::new(dma, dma1).0));

    }

    pub fn write_dma(&self, args: fmt::Arguments) {

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

        self.tx.replace(Some(tx));
        self.stream.replace(Some(stream));
    }
}

impl ConsoleDriver for Console {
    #[inline(always)]
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        self.tx.borrow_mut().as_mut().unwrap().write_fmt(args)
    }

    #[inline(always)]
    fn write_char(&self, c: char) -> fmt::Result {
        self.tx.borrow_mut().as_mut().unwrap().write_char(c)
    }

    #[inline(always)]
    fn read(&self) -> Option<u8> {
        self.rx.borrow_mut().as_mut().unwrap().read().ok()
    }

    #[inline(always)]
    fn flush(&self) {
        self.tx.borrow_mut().as_mut().unwrap().flush().unwrap()
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
