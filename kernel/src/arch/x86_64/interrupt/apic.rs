use common::*;
use arch::init::{LOCAL_APIC_PAGE_VADDR, IO_APIC_PAGE_VADDR};
use util::{Mutex};
use super::{InterruptVector};

#[derive(Debug)]
pub struct LocalAPIC {
    address: VAddr,
}

#[derive(Debug)]
pub struct IOAPIC {
    address: VAddr,
}

pub static LOCAL_APIC: Mutex<LocalAPIC> = Mutex::new(LocalAPIC {
    address: LOCAL_APIC_PAGE_VADDR
});

pub static IO_APIC: Mutex<IOAPIC> = Mutex::new(IOAPIC {
    address: IO_APIC_PAGE_VADDR
});

impl LocalAPIC {
    unsafe fn read(&self, reg: u32) -> u32 {
        use core::intrinsics::{volatile_load};
        volatile_load((self.address.into(): usize + reg as usize) as *const u32)
    }

    unsafe fn write(&mut self, reg: u32, value: u32) {
        use core::intrinsics::{volatile_store};
        volatile_store((self.address.into(): usize + reg as usize) as *mut u32, value);
    }

    pub fn id(&self) -> u32 {
        unsafe { self.read(0x20) }
    }

    pub fn version(&self) -> u32 {
        unsafe { self.read(0x30) }
    }

    pub fn siv(&self) -> u32 {
        unsafe { self.read(0xF0) }
    }

    pub fn set_siv(&mut self, value: u32) {
        unsafe { self.write(0xF0, value) }
    }

    pub fn eoi(&mut self) {
        unsafe { self.write(0xB0, 0) }
    }

    pub fn enable_timer(&mut self) {
        unsafe {
            self.write(0x3E0, 0x3);
            self.write(0x380, 0x10000);
            self.write(0x320, (1<<17) | 0x40);
            log!("timer register is 0b{:b}", self.read(0x320));
        }
    }

    pub fn error_status(&self) -> u32 {
        unsafe { self.read(0x280) }
    }
}

impl IOAPIC {
    unsafe fn read(&self, reg: u32) -> u32 {
        use core::intrinsics::{volatile_load, volatile_store};
        volatile_store((self.address.into(): usize + 0x0 as usize) as *mut u32, reg);
        volatile_load((self.address.into(): usize + 0x10 as usize) as *const u32)
    }

    unsafe fn write(&mut self, reg: u32, value: u32) {
        use core::intrinsics::{volatile_load, volatile_store};
        volatile_store((self.address.into(): usize + 0x0 as usize) as *mut u32, reg);
        volatile_store((self.address.into(): usize + 0x10 as usize) as *mut u32, value);
    }

    pub fn id(&self) -> u32 {
        unsafe { self.read(0x0) }
    }

    pub fn version(&self) -> u32 {
        unsafe { self.read(0x1) }
    }

    pub fn arbitration_id(&self) -> u32 {
        unsafe { self.read(0x2) }
    }

    pub fn set_irq(&mut self, irq: u8, apic_id: u8, vector: InterruptVector) {
        let vector = vector as u8;

        let low_index: u32 = 0x10 + (irq as u32) * 2;
        let high_index: u32 = 0x10 + (irq as u32) * 2 + 1;

        let mut high = unsafe { self.read(high_index) };
        high &= !0xff000000;
        high |= (apic_id as u32) << 24;
        unsafe { self.write(high_index, high) };

        let mut low = unsafe { self.read(low_index) };
        low &= !(1<<16);
        low &= !(1<<11);
        low &= !0x700;
        low &= !0xff;
        low |= (vector as u32);
        unsafe { self.write(low_index, low) };
    }
}
