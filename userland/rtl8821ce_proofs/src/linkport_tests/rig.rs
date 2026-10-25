// NONOS Operating System (AGPL-3.0-or-later)

//! The addresses, the DMA memory and the link built over them.

use std::cell::RefCell;
use std::rc::Rc;

use super::card::Card;
use super::rxbuf::RxBuf;
use crate::fw::dma::DmaMem;
use crate::link::RtlLink;

pub(crate) const STA_MAC: [u8; 6] = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
pub(crate) const AP_MAC: [u8; 6] = [0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

pub(crate) const PTK: [u8; 16] = [0x5A; 16];

// A modeled DMA region: readable and writable bytes with a bus address. The
// backing store is shared so a test can inspect what the driver laid down after
// the link has taken ownership of the region.
#[derive(Clone)]
pub(crate) struct Dma {
    pub(crate) mem: Rc<RefCell<Vec<u8>>>,
    pub(crate) dev: u64,
}

impl Dma {
    pub(crate) fn new(len: usize, dev: u64) -> Self {
        Self { mem: Rc::new(RefCell::new(vec![0u8; len])), dev }
    }
}

impl DmaMem for Dma {
    fn capacity(&self) -> usize {
        self.mem.borrow().len()
    }
    fn device_addr(&self) -> u64 {
        self.dev
    }
    fn write_bytes(&self, offset: usize, src: &[u8]) {
        self.mem.borrow_mut()[offset..offset + src.len()].copy_from_slice(src);
    }
}

// Build a link over fresh DMA. Returns the link, a handle on the TX buffer store
// so a test can read the descriptor the driver emitted, and the card handle.
pub(crate) fn link_with(card: Card, rx: RxBuf) -> (RtlLink<Card, Dma, RxBuf>, Dma) {
    let tx_ring = Dma::new(1 << 12, 0x1000_0000);
    let tx_buffers = Dma::new(1 << 16, 0x2000_0000);
    let rx_ring = Dma::new(1 << 12, 0x3000_0000);
    let tx_handle = tx_buffers.clone();
    let link = RtlLink::new(card, tx_ring, tx_buffers, rx_ring, rx, STA_MAC);
    (link, tx_handle)
}

// A minimal Ethernet frame to `dst` from `src` with one payload byte.
pub(crate) fn eth_frame(dst: [u8; 6], src: [u8; 6], ethertype: [u8; 2], payload: &[u8]) -> Vec<u8> {
    let mut e = Vec::new();
    e.extend_from_slice(&dst);
    e.extend_from_slice(&src);
    e.extend_from_slice(&ethertype);
    e.extend_from_slice(payload);
    e
}
