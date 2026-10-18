// NONOS Operating System (AGPL-3.0-or-later)

//! The modeled device the receive-path proofs program.

use core::cell::RefCell;

use crate::regs::Mmio;

pub(super) struct Dev {
    store: RefCell<[u32; 0x3000]>,
}

impl Dev {
    pub(super) fn new() -> Self {
        Self { store: RefCell::new([0u32; 0x3000]) }
    }
    pub(super) fn get(&self, off: usize) -> u32 {
        self.store.borrow()[off]
    }
    pub(super) fn seed(&self, off: usize, v: u32) {
        self.store.borrow_mut()[off] = v;
    }
}

impl Mmio for Dev {
    fn read8(&self, off: usize) -> u8 {
        self.store.borrow()[off] as u8
    }
    fn write8(&self, off: usize, v: u8) {
        self.store.borrow_mut()[off] = v as u32;
    }
    fn read16(&self, off: usize) -> u16 {
        self.store.borrow()[off] as u16
    }
    fn write16(&self, off: usize, v: u16) {
        self.store.borrow_mut()[off] = v as u32;
    }
    fn read32(&self, off: usize) -> u32 {
        self.store.borrow()[off]
    }
    fn write32(&self, off: usize, v: u32) {
        self.store.borrow_mut()[off] = v;
    }
}

pub(super) const REG_RXPSEL: usize = 0x0808;
pub(super) const REG_CCK_CHECK: usize = 0x0454;
pub(super) const REG_RFECTL: usize = 0x0CB8;
pub(super) const REG_ACBB0: usize = 0x0948;
pub(super) const RX_PSEL_RST: u32 = (1 << 28) | (1 << 29);
