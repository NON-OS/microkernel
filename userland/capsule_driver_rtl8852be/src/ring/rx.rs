// NONOS Operating System (AGPL-3.0-or-later)

//! A receive ring (RXQ or RPQ): a page of buffer descriptors, then one
//! buffer per descriptor. The card fills buffers from its read pointer and
//! advances it; the host takes them from its own pointer up to the card's
//! (`rtw89_pci_rxbd_recalc`, with `rx_ring_eq_is_full` false on the 8852B)
//! and writes its pointer back when done (`rtw89_pci_rxbd_deliver`).

use super::addrs::hw_idx;
use super::bd::{rx_bd, RX_BD_SIZE};
use crate::dma::DmaMem;
use crate::regs::Mmio;

/// The descriptor page at the head of the region.
pub const BD_PAGE: usize = 4096;

pub struct RxRing<M: DmaMem> {
    pub mem: M,
    /// The ring's (num, idx, desa_l) registers.
    pub regs: (u32, u32, u32),
    pub len: u32,
    pub buf: usize,
    pub wp: u32,
}

impl<M: DmaMem> RxRing<M> {
    /// A ring of `len` buffers of `buf` bytes over `mem`, every descriptor
    /// written to hand its buffer to the card (`rtw89_pci_init_rx_bd`).
    pub fn new(mem: M, regs: (u32, u32, u32), len: u32, buf: usize) -> Option<Self> {
        let fits = len >= 2 && len as usize <= BD_PAGE / RX_BD_SIZE && buf <= 0xffff;
        if !fits || mem.capacity() < bytes_for(len, buf) {
            return None;
        }
        let ring = Self { mem, regs, len, buf, wp: 0 };
        for i in 0..len {
            let dma = ring.mem.bus_at(ring.buf_off(i))?;
            ring.mem.write(i as usize * RX_BD_SIZE, &rx_bd(buf as u16, dma));
        }
        Some(ring)
    }

    pub fn buf_off(&self, i: u32) -> usize {
        BD_PAGE + i as usize * self.buf
    }

    /// Buffers the card has filled since the host's pointer, or `None` on a
    /// dead read or a pointer past the ring.
    pub fn ready(&self, r: &impl Mmio) -> Option<u32> {
        let v = r.read32(self.regs.1);
        let rp = hw_idx(v);
        if v == u32::MAX || rp >= self.len {
            return None;
        }
        Some(if rp >= self.wp { rp - self.wp } else { self.len - (self.wp - rp) })
    }

    /// Done with the buffer at the host's pointer: clear its head, so a stale
    /// info word is never read as a new frame, and step on.
    pub fn advance(&mut self) {
        self.mem.write(self.buf_off(self.wp), &[0u8; 4]);
        self.wp = (self.wp + 1) % self.len;
    }

    /// Give the card back every buffer taken: the host pointer to the index.
    pub fn release(&self, r: &impl Mmio) {
        r.write16(self.regs.1, self.wp as u16);
    }
}

pub const fn bytes_for(len: u32, buf: usize) -> usize {
    BD_PAGE + len as usize * buf
}
