// NONOS Operating System (AGPL-3.0-or-later)

//! What a queued frame's command and byte count entry are in the family's
//! layout. From AX210 on the command is `iwl_tx_cmd_gen3` and the entry
//! counts bytes, the fetch chunks in bits 14-15; on the 22000 family it is
//! `iwl_tx_cmd_gen2` and the entry counts dwords, the chunks in bits 12-15
//! (Linux v6.12 `iwl_mvm_set_tx_params`, `iwl_pcie_gen2_update_byte_tbl`).

use super::super::csr::CsrLayout;
use super::super::tx_cmd::{header, header_gen2, TX_CMD_GEN2_HDR, TX_CMD_GEN3_HDR};

/// The transmit command for a frame, and its length.
pub(super) fn tx_head(
    csr: CsrLayout,
    frame_len: usize,
    hdr_len: usize,
    rate_n_flags: u32,
) -> Option<([u8; TX_CMD_GEN3_HDR], usize)> {
    if csr == CsrLayout::Gen2 {
        let mut out = [0u8; TX_CMD_GEN3_HDR];
        out[..TX_CMD_GEN2_HDR].copy_from_slice(&header_gen2(frame_len, hdr_len, rate_n_flags)?);
        return Some((out, TX_CMD_GEN2_HDR));
    }
    Some((header(frame_len, hdr_len, rate_n_flags)?, TX_CMD_GEN3_HDR))
}

/// The byte count entry for a `len`-byte frame in a descriptor of `buffers`
/// buffers: `filled_tfd_size` is 2 bytes and 10 per buffer, fetched in
/// 64-byte chunks, the count of chunks less one.
pub fn byte_count(csr: CsrLayout, len: usize, buffers: u16) -> u16 {
    let chunks = (2 + 10 * buffers).div_ceil(64) - 1;
    match csr {
        CsrLayout::Gen2 => len.div_ceil(4) as u16 | chunks << 12,
        CsrLayout::V1 | CsrLayout::V2 => len as u16 | chunks << 14,
    }
}
