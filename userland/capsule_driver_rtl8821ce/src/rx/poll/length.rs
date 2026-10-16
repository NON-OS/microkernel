// NONOS Operating System (AGPL-3.0-or-later)

//! The length-only poll the proofs drive.

use alloc::vec::Vec;

use super::super::ring::RxState;
use super::info::poll_one_info;
use crate::fw::dma::DmaMem;
use crate::link::RxBuffers;
use crate::regs::Mmio;

/// Copy the next deliverable frame into `out` and return its length, or `None`
/// if no complete data frame is waiting. `ring` is the buffer-descriptor ring;
/// `buffers` is the CPU view of the packet buffer and `buffers_dev` its bus
/// address (used to re-arm a consumed slot). The driver reads frames with
/// [`poll_one_info`]; this length-only form is what the proofs drive, over a
/// copy of `buffers`, so the slots it clears are its own.
#[cfg(test)]
pub fn poll_one<M, R>(
    mmio: &M,
    ring: &R,
    buffers: &[u8],
    buffers_dev: u64,
    state: &mut RxState,
    out: &mut [u8],
) -> Option<usize>
where
    M: Mmio,
    R: DmaMem,
{
    let mut copy = Copy { mem: buffers.to_vec(), dev: buffers_dev };
    poll_one_info(mmio, ring, &mut copy, state, out).map(|(n, _)| n)
}

// Plain memory as the receive region.
struct Copy {
    mem: Vec<u8>,
    dev: u64,
}

impl RxBuffers for Copy {
    fn bytes(&self) -> &[u8] {
        &self.mem
    }
    fn device_addr(&self) -> u64 {
        self.dev
    }
    fn clear(&mut self, offset: usize, len: usize) {
        let end = offset.saturating_add(len).min(self.mem.len());
        self.mem.get_mut(offset..end).into_iter().for_each(|s| s.fill(0));
    }
}
