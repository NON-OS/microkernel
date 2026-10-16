// NONOS Operating System (AGPL-3.0-or-later)

//! Taking one frame off the ring, with its parsed descriptor.

use super::super::desc::{parse, rearm, RxInfo};
use super::super::regs::REG_RXBD_IDX_MPDUQ;
use super::super::ring::{RxState, RX_BUF_STRIDE};
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::tx::regs::{TRX_BD_HW_IDX_MASK, TRX_BD_HW_IDX_SHIFT, TRX_BD_IDX_MASK};

/// As [`poll_one`], also returning the frame's parsed descriptor, so the data
/// path knows whether the chip decrypted it.
pub fn poll_one_info<M, R>(
    mmio: &M,
    ring: &R,
    buffers: &[u8],
    buffers_dev: u64,
    state: &mut RxState,
    out: &mut [u8],
) -> Option<(usize, RxInfo)>
where
    M: Mmio,
    R: DmaMem,
{
    let idx = mmio.read32(REG_RXBD_IDX_MPDUQ);
    let hw_wp = ((idx & TRX_BD_HW_IDX_MASK) >> TRX_BD_HW_IDX_SHIFT) % state.len;

    while state.ready(hw_wp) > 0 {
        let slot = state.rp;
        let buf_off = RxState::buffer_offset(slot);
        let slot_bytes = buffers.get(buf_off..).map(|b| &b[..b.len().min(RX_BUF_STRIDE)]);
        let info = slot_bytes.and_then(parse);

        // Decide whether this slot carries a frame we can hand up. A CRC or ICV
        // error is counted, not just skipped: an undecryptable reply (bad group
        // key) shows up here rather than as a silent gap.
        let delivered = match (info, slot_bytes) {
            (Some(i), Some(bytes))
                if i.deliverable() && i.total_len() <= bytes.len() && i.pkt_len <= out.len() =>
            {
                let start = i.frame_offset();
                out[..i.pkt_len].copy_from_slice(&bytes[start..start + i.pkt_len]);
                Some((i.pkt_len, i))
            }
            (Some(i), _) if i.crc_err || i.icv_err => {
                state.err_drops = state.err_drops.wrapping_add(1);
                None
            }
            _ => None,
        };

        // Re-arm the slot and hand it back to the card.
        let dma = buffers_dev + buf_off as u64;
        ring.write_bytes(RxState::desc_offset(slot), &rearm(dma, RX_BUF_STRIDE));
        state.advance();
        mmio.write16(REG_RXBD_IDX_MPDUQ, (state.rp & TRX_BD_IDX_MASK) as u16);

        if delivered.is_some() {
            return delivered;
        }
    }
    None
}
