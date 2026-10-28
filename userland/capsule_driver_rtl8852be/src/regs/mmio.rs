// NONOS Operating System (AGPL-3.0-or-later)

//! The register surface every bring-up step is written against. The card
//! backs it with volatile MMIO (`window::Regs`); the host proofs back it
//! with a modelled chip, so the exact program a step writes is checked
//! without a card.

pub trait Mmio {
    fn read8(&self, off: u32) -> u8;
    fn write8(&self, off: u32, val: u8);
    fn read16(&self, off: u32) -> u16;
    fn write16(&self, off: u32, val: u16);
    fn read32(&self, off: u32) -> u32;
    fn write32(&self, off: u32, val: u32);
}
