// NONOS Operating System (AGPL-3.0-or-later)

//! Register access for the Realtek 8821CE. rtw88 works predominantly in 8-bit
//! MAC registers, so the byte accessors are primary; the power-on sequence and
//! everything above it is written against the `Mmio` trait rather than raw
//! volatile access, so the host proofs can drive the real code with a modeled
//! device and check the exact register program without a card.

/// The register surface the driver logic is written against. A real device
/// backs it with volatile MMIO; a proof backs it with a modeled register file.
pub trait Mmio {
    fn read8(&self, off: usize) -> u8;
    fn write8(&self, off: usize, val: u8);
    fn read16(&self, off: usize) -> u16;
    fn write16(&self, off: usize, val: u16);
    fn read32(&self, off: usize) -> u32;
    fn write32(&self, off: usize, val: u32);
}
