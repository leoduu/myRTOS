
use core::{cell::RefCell, ops::DerefMut};
use cortex_m::peripheral::{MPU, SCB};
use cortex_m_rt::exception;

pub struct MPUControl {
    mpu: RefCell<Option<MPU>>
}

impl MPUControl {
    pub const fn new() -> Self {
        Self { 
            mpu: RefCell::new(None),
        }
    }

    pub unsafe fn init(&self, mpu: MPU, scb: &mut SCB) {


        fn log2minus1(sz: u32) -> u32 {
            for i in 5..=31 {
                if sz == (1 << i) {
                    return i - 1;
                }
            }
            panic!("Unknown SDRAM memory region size!");
        }

        cortex_m::asm::dsb();

        /* Disable the MPU and clear the control register*/
        mpu.ctrl.write(0);

        const RAM_NUMBER0:          u32 = 0x00;
        const RAM_SIZE:             u32 = 128 * 1024;
        const RAM_BASE_ADDRESS:     u32 = 0x2000_0000;
        const RAM_FULL_ACCESS:      u32 = 0x03 << 24;
        const RAM_CACHEABLE:        u32 = 0x01 << 17;
        const RAM_WRITE_BACK:       u32 = 0x01 << 16;
        const RAM_ENABLE:           u32 = 0x01;

        mpu.rnr.write(RAM_NUMBER0);
        mpu.rbar.write(RAM_BASE_ADDRESS);
        mpu.rasr.write(
              RAM_FULL_ACCESS
            | RAM_CACHEABLE
            | RAM_WRITE_BACK
            | (log2minus1(RAM_SIZE) << 1)
            | RAM_ENABLE
        );

        const SRAM_NUMBER0:         u32 = 0x01;
        const SRAM_SIZE:            u32 = 128 * 1024;
        const SRAM_BASE_ADDRESS:    u32 = 0x3000_0000;
        const SRAM_FULL_ACCESS:     u32 = 0x03 << 24;
        const SRAM_CACHEABLE:       u32 = 0x01 << 17;
        const SRAM_WRITE_BACK:      u32 = 0x01 << 16;
        const SRAM_ENABLE:          u32 = 0x01;

        mpu.rnr.write(SRAM_NUMBER0);
        mpu.rbar.write(SRAM_BASE_ADDRESS);
        mpu.rasr.write(
              SRAM_FULL_ACCESS
            | SRAM_CACHEABLE
            | SRAM_WRITE_BACK
            | (log2minus1(SRAM_SIZE) << 1)
            | SRAM_ENABLE
        );

        const FLASH_NUMBER1:        u32 = 0x02;
        const FLASH_SIZE:           u32 = 128 * 1024;
        const FLASH_BASE_ADDRESS:   u32 = 0x800_0000;
        const FLASH_READ_ONLY:      u32 = 0x03 << 24;
        const FLASH_CACHEABLE:      u32 = 0x01 << 17;
        const FLASH_WRITE_BACK:     u32 = 0x01 << 16;
        const FLASH_ENABLE:         u32 = 0x01;

        mpu.rnr.write(FLASH_NUMBER1);
        mpu.rbar.write(FLASH_BASE_ADDRESS);
        mpu.rasr.write(
              FLASH_READ_ONLY
            | FLASH_CACHEABLE
            | FLASH_WRITE_BACK
            | (log2minus1(FLASH_SIZE) << 1)
            | FLASH_ENABLE
        );

        const SYSTEM_NUMBER1:        u32 = 0x03;
        const SYSTEM_SIZE:           u32 = 1024 * 1024;
        const SYSTEM_BASE_ADDRESS:   u32 = 0xE000_0000;
        const SYSTEM_READ_ONLY:      u32 = 0x03 << 24;
        const SYSTEM_CACHEABLE:      u32 = 0x01 << 17;
        const SYSTEM_WRITE_BACK:     u32 = 0x01 << 16;
        const SYSTEM_ENABLE:         u32 = 0x01;

        mpu.rnr.write(SYSTEM_NUMBER1);
        mpu.rbar.write(SYSTEM_BASE_ADDRESS);
        mpu.rasr.write(
              SYSTEM_READ_ONLY
            | SYSTEM_CACHEABLE
            | SYSTEM_WRITE_BACK
            | (log2minus1(SYSTEM_SIZE) << 1)
            | SYSTEM_ENABLE
        );

        const MPU_ENABLE: u32 = 0x01;
        const MPU_DEFAULT_MMAP_FOR_PRIVILEGED: u32 = 0x1 << 2;
        const MEMFAULTENA: u32 = 1 << 16;

        // Enable
        mpu.ctrl.modify(|r| r | MPU_DEFAULT_MMAP_FOR_PRIVILEGED | MPU_ENABLE);

        scb.shcsr.modify(|r| r | MEMFAULTENA);

        // Ensure MPU settings take effect
        cortex_m::asm::dsb();
        cortex_m::asm::isb();   

        self.mpu.replace(Some(mpu));
    }

    #[inline(never)]
    pub unsafe fn mpu_disable(&self) {

        cortex_m::asm::dsb();

        /* Disable the MPU and clear the control register*/
        if let Some(mpu) = self.mpu.borrow_mut().deref_mut() {
            mpu.ctrl.write(0);
        }
    }
}

#[exception]
fn MemoryManagement() {

    cortex_m::interrupt::disable();

    use crate::println;
    println!("MemoryManagement");

    loop {}
}
