// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's DMA region on the host: memory a test allocated, whose
//! address the model HBA is given as the device address too. The driver's
//! checked copies (`region_io`) run on it unchanged.

pub struct DmaRegion {
    addr: u64,
    len: u64,
}

impl DmaRegion {
    pub fn at(addr: u64, len: u64) -> Self {
        Self { addr, len }
    }

    pub const fn user_va(&self) -> u64 {
        self.addr
    }

    pub const fn device_addr(&self) -> u64 {
        self.addr
    }

    pub const fn len(&self) -> u64 {
        self.len
    }
}
