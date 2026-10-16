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

//! Queue one frame on the BE ring. The hardware read index is refreshed first
//! so a full ring is not overrun; the frame's TX descriptor and body go into its
//! packet-buffer slot, the rest of the slot is cleared, the slot's buffer
//! descriptor goes into the ring, and a 16-bit write of the host write index
//! kicks the queue: rtw88 `rtw_pci_tx_write_data` and
//! `rtw_pci_tx_kick_off_queue` (pci.c), checked in `rtl8821ce_proofs`.

use super::desc::{self, FrameMeta, TXDESC_LEN};
use super::regs::{REG_TXBD_IDX_BEQ, TRX_BD_HW_IDX_MASK, TRX_BD_HW_IDX_SHIFT, TRX_BD_IDX_MASK};
use super::ring::{TxState, TX_BUF_STRIDE};
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

/// Queue `frame` on the BE ring with descriptor inputs `meta`. Returns false if
/// the ring is full or the frame does not fit a buffer slot.
pub fn enqueue<M, R, B>(
    mmio: &M,
    ring: &R,
    buffers: &B,
    state: &mut TxState,
    frame: &[u8],
    meta: &FrameMeta,
) -> bool
where
    M: Mmio,
    R: DmaMem,
    B: DmaMem,
{
    // Refresh the hardware read index so freed descriptors are visible.
    let idx = mmio.read32(REG_TXBD_IDX_BEQ);
    state.set_hw_rp((idx & TRX_BD_HW_IDX_MASK) >> TRX_BD_HW_IDX_SHIFT);

    if frame.is_empty() || frame.len() + TXDESC_LEN > TX_BUF_STRIDE || state.avail() == 0 {
        return false;
    }

    let slot = state.wp;
    let buf_off = TxState::buffer_offset(slot);
    let d = desc::frame(frame.len(), meta);
    buffers.write_bytes(buf_off, &d);
    buffers.write_bytes(buf_off + TXDESC_LEN, frame);
    // No byte of an earlier, longer frame stays in the slot.
    let tail = TX_BUF_STRIDE - TXDESC_LEN - frame.len();
    buffers.write_bytes(buf_off + TXDESC_LEN + frame.len(), &[0u8; TX_BUF_STRIDE][..tail]);

    let bd = crate::ring::bufdesc::pair(
        buffers.device_addr() + buf_off as u64,
        TXDESC_LEN,
        frame.len(),
        false,
    );
    ring.write_bytes(TxState::desc_offset(slot), &bd);

    state.advance();
    // A 16-bit write to the low half of the index register sets the host write
    // index and kicks the queue, leaving the hardware read index untouched.
    mmio.write16(REG_TXBD_IDX_BEQ, (state.wp & TRX_BD_IDX_MASK) as u16);
    true
}
