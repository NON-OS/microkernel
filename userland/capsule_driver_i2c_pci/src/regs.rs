#[derive(Clone, Copy)]
pub struct Regs {
    base: *mut u8,
}

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { base: base as *mut u8 }
    }

    pub fn read32(&self, offset: u64) -> u32 {
        unsafe { nonos_libc::mmio_read32(self.base.add(offset as usize) as usize) }
    }

    pub fn write32(&self, offset: u64, value: u32) {
        unsafe { nonos_libc::mmio_write32(self.base.add(offset as usize) as usize, value) }
    }
}
