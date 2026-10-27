// NONOS Operating System (AGPL-3.0-or-later)
//! Host memory the modelled card reaches by bus address: each region the
//! driver is given is registered here, and the card's DMA reads and writes
//! go through `read` and `write`, which refuse anything outside a region.

use crate::dma::DmaMem;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Mem {
    pub base: u64,
    pub buf: Rc<RefCell<Vec<u8>>>,
}

fn fits(cap: usize, off: usize, len: usize) -> bool {
    off.checked_add(len).is_some_and(|end| end <= cap)
}

impl DmaMem for Mem {
    fn capacity(&self) -> usize {
        self.buf.borrow().len()
    }
    fn device_addr(&self) -> u64 {
        self.base
    }
    fn write(&self, offset: usize, src: &[u8]) {
        let mut b = self.buf.borrow_mut();
        if fits(b.len(), offset, src.len()) {
            b[offset..offset + src.len()].copy_from_slice(src);
        }
    }
    fn read(&self, offset: usize, dst: &mut [u8]) -> bool {
        let b = self.buf.borrow();
        if !fits(b.len(), offset, dst.len()) {
            return false;
        }
        dst.copy_from_slice(&b[offset..offset + dst.len()]);
        true
    }
}

/// The region holding `[addr, addr + len)` whole, and the offset in it.
fn find(bus: &[Mem], addr: u64, len: usize) -> Option<(&Mem, usize)> {
    bus.iter().find_map(|m| {
        let off = addr.checked_sub(m.base)? as usize;
        fits(m.capacity(), off, len).then_some((m, off))
    })
}

/// The card's DMA read; `None` outside every region.
pub fn read(bus: &[Mem], addr: u64, len: usize) -> Option<Vec<u8>> {
    let (m, off) = find(bus, addr, len)?;
    let mut v = vec![0; len];
    m.read(off, &mut v).then_some(v)
}

/// The card's DMA write; false outside every region.
pub fn write(bus: &[Mem], addr: u64, data: &[u8]) -> bool {
    let Some((m, off)) = find(bus, addr, data.len()) else { return false };
    m.write(off, data);
    true
}

/// A fresh zeroed region of `bytes`, below 4 GiB, after every other one.
pub fn alloc(bus: &mut Vec<Mem>, bytes: usize) -> Mem {
    let base = bus.last().map_or(0x1000_0000, |m| m.base + m.capacity() as u64 + 0x1000);
    let m = Mem { base, buf: Rc::new(RefCell::new(vec![0; bytes])) };
    bus.push(m.clone());
    m
}
