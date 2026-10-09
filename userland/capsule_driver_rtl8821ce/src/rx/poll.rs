// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Take the next received 802.11 frame off the RX ring. The hardware write index
//! says how many slots are ready; each is parsed, and firmware-command, error
//! and oversized frames are skipped (their slots re-armed) until a deliverable
//! frame is found. Its body is copied out, the slot is re-armed, the host read
//! index advances and is written back to hand the slot to the card. This is the
//! read half of rtw88 `rtw_pci_rx_napi`, shaped as a single-frame poll so it
//! backs the `poll_rx` half of the net_core link contract. Checked against a
//! modeled device and DMA memory in `rtl8821ce_proofs`.
//!
//! Everything the device wrote is untrusted: the descriptor's length, driver
//! info size and shift are only used once the frame they describe is shown to
//! lie inside its own slot and inside the mapped buffer, and a slot whose
//! buffer the mapping does not cover is skipped rather than read.

use super::desc::{parse, rearm, RxInfo};
use super::regs::REG_RXBD_IDX_MPDUQ;
use super::ring::{RxState, RX_BUF_STRIDE};
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::tx::regs::{TRX_BD_HW_IDX_MASK, TRX_BD_HW_IDX_SHIFT, TRX_BD_IDX_MASK};

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
