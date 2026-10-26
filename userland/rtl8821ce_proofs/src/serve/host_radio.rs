// NONOS Operating System (AGPL-3.0-or-later)

//! A radio over host memory, as the serve stage holds one: the register
//! window an inert page (a write reads back, nothing else happens), the rings
//! and buffers plain memory. Frames are staged straight into the receive ring.

use super::radio::{Radio, RadioUp};
use crate::fw::dma::Grant;
use crate::link::{RtlKeys, RtlLink};
use crate::regs::{Mmio, Regs};
use crate::ring::TX_BUF_DESC_SIZE;
use crate::rx::regs::{REG_RXBD_IDX_MPDUQ, RX_BUF_DESC_SIZE};
use crate::rx::ring::{RxState, RX_BUF_STRIDE, RX_DESC_COUNT};
use crate::tx::regs::TRX_BD_HW_IDX_SHIFT;
use crate::tx::ring::{TX_BUF_STRIDE, TX_DESC_COUNT};

const BAR: usize = 0x1_0000;

// Zeroed memory that lives for the rest of the run, eight-byte aligned.
fn leak(bytes: usize) -> u64 {
    Box::leak(vec![0u64; bytes.div_ceil(8)].into_boxed_slice()).as_mut_ptr() as u64
}

fn grant(bytes: usize, dev: u64) -> Grant {
    // SAFETY: `leak` gave `bytes` zeroed bytes that are never freed.
    unsafe { Grant::new(leak(bytes), dev, bytes) }
}

/// The radio, its registers, and where its receive buffers sit.
pub(super) struct Host {
    pub(super) regs: Regs,
    rx_va: u64,
    staged: u32,
}

pub(super) fn host_radio() -> (Radio, Host) {
    // SAFETY: `leak` gave BAR zeroed bytes that are never freed.
    let regs = unsafe { Regs::new(leak(BAR), BAR as u64) };
    let rx_bytes = RX_DESC_COUNT as usize * RX_BUF_STRIDE;
    let rx_va = leak(rx_bytes);
    // SAFETY: as above, `rx_bytes` of them.
    let rx_buffers = unsafe { Grant::new(rx_va, 0x4000_0000, rx_bytes) };
    let link = RtlLink::new(
        regs,
        grant(TX_DESC_COUNT as usize * TX_BUF_DESC_SIZE, 0x1000_0000),
        grant(TX_DESC_COUNT as usize * TX_BUF_STRIDE, 0x2000_0000),
        grant(RX_DESC_COUNT as usize * RX_BUF_DESC_SIZE, 0x3000_0000),
        rx_buffers,
        [0x02, 0x11, 0x22, 0x33, 0x44, 0x55],
    );
    let up = RadioUp { link, keys: RtlKeys::new(regs), regs };
    (Radio::Up(Box::new(up)), Host { regs, rx_va, staged: 0 })
}

impl Host {
    /// Stage `frame` behind a receive descriptor whose first word is `w0`
    /// (its length alone for a clean one) in the next slot, and move the
    /// card's write index past it. The frame is cut to the slot.
    pub(super) fn stage(&mut self, w0: u32, frame: &[u8]) {
        let slot = self.staged % RX_DESC_COUNT;
        let len = frame.len().min(RX_BUF_STRIDE - 24);
        let base = (self.rx_va as usize + RxState::buffer_offset(slot)) as *mut u8;
        let w0 = w0.to_le_bytes();
        // SAFETY: the slot lies inside the receive buffers `host_radio` leaked.
        unsafe {
            core::ptr::write_bytes(base, 0, 24);
            core::ptr::copy_nonoverlapping(w0.as_ptr(), base, 4);
            core::ptr::copy_nonoverlapping(frame.as_ptr(), base.add(24), len);
        }
        self.staged += 1;
        let rp = self.regs.read32(REG_RXBD_IDX_MPDUQ) & 0xFFFF;
        let wp = self.staged % RX_DESC_COUNT;
        self.regs.write32(REG_RXBD_IDX_MPDUQ, (wp << TRX_BD_HW_IDX_SHIFT) | rp);
    }
}
