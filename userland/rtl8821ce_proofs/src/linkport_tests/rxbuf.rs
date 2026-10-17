// NONOS Operating System (AGPL-3.0-or-later)

//! The receive buffers, and frames staged in them.

use crate::link::RxBuffers;
use crate::rx::ring::{RxState, RX_BUF_STRIDE, RX_DESC_COUNT};

// The receive buffers as a CPU-readable region. The poll path only reads them, so
// a plain vector suffices; the harness stages a frame before the link is built.
pub(crate) struct RxBuf {
    pub(crate) mem: Vec<u8>,
    pub(crate) dev: u64,
}

impl RxBuf {
    pub(crate) fn new(dev: u64) -> Self {
        Self { mem: vec![0u8; RX_DESC_COUNT as usize * RX_BUF_STRIDE], dev }
    }

    // Stage a decrypted 802.11 MPDU behind a clean data descriptor in `slot`.
    pub(crate) fn stage(&mut self, slot: u32, mpdu: &[u8]) {
        self.stage_w0(slot, mpdu, 0);
    }

    // Stage a frame behind a descriptor with extra word-0 bits (the security
    // type, SWDEC).
    pub(crate) fn stage_w0(&mut self, slot: u32, mpdu: &[u8], w0_extra: u32) {
        let base = RxState::buffer_offset(slot);
        let mut d = rx_desc(mpdu.len() as u16);
        let w0 = u32::from_le_bytes([d[0], d[1], d[2], d[3]]) | w0_extra;
        d[0..4].copy_from_slice(&w0.to_le_bytes());
        self.mem[base..base + 24].copy_from_slice(&d);
        self.mem[base + 24..base + 24 + mpdu.len()].copy_from_slice(mpdu);
    }
}

impl RxBuffers for RxBuf {
    fn bytes(&self) -> &[u8] {
        &self.mem
    }
    fn device_addr(&self) -> u64 {
        self.dev
    }
    fn clear(&mut self, offset: usize, len: usize) {
        let end = offset.saturating_add(len).min(self.mem.len());
        self.mem[offset.min(end)..end].fill(0);
    }
}

// A clean data RX descriptor: length only, no driver info, no shift, no errors,
// not a firmware command. The frame body sits at the fixed 24-byte offset.
fn rx_desc(pkt_len: u16) -> [u8; 24] {
    let w0 = (pkt_len as u32) & 0x3FFF;
    let mut d = [0u8; 24];
    d[0..4].copy_from_slice(&w0.to_le_bytes());
    d
}
