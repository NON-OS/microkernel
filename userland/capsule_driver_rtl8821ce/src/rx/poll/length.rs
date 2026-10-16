// NONOS Operating System (AGPL-3.0-or-later)

//! The length-only poll the proofs drive.

use super::super::ring::RxState;
use super::info::poll_one_info;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

/// Copy the next deliverable frame into `out` and return its length, or `None`
/// if no complete data frame is waiting. `ring` is the buffer-descriptor ring;
/// `buffers` is the CPU view of the packet buffer and `buffers_dev` its bus
/// address (used to re-arm a consumed slot). The driver reads frames with
/// [`poll_one_info`]; this length-only form is what the proofs drive.
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
    poll_one_info(mmio, ring, buffers, buffers_dev, state, out).map(|(n, _)| n)
}
