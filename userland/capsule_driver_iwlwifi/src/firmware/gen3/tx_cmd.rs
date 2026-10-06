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

//! Wrap an already-built 802.11 frame in the gen3 transmit command the firmware
//! TX queue consumes. The 28-byte header (struct iwl_tx_cmd_gen3 in
//! fw/api/tx.h) carries the payload length, the transmit flags and the rate;
//! the DRAM security info is left zero for the firmware to fill, and the frame
//! follows the header. The frame handed in is the CCMP-protected 802.11 frame
//! from the data path, so encryption is already done in software. Offsets are
//! pinned by the gen3 proofs.
//!
//! The header a queued frame carries (`header`) is the one Linux v6.12
//! mvm/tx.c `iwl_mvm_set_tx_params` writes on an AX210-family part for a frame
//! sent at a rate it chose and with no key the firmware is to apply: `len` the
//! whole frame, `flags` `IWL_TX_FLAGS_CMD_RATE` and `IWL_TX_FLAGS_ENCRYPT_DIS`,
//! `offload_assist` 4 with `TX_CMD_OFFLD_PAD` when the 802.11 header is not a
//! multiple of four bytes (the transport pads it, `iwl_mvm_tx_csum`), the rate
//! word at 16, and the security info and reserved bytes zero.

/// Command id for a transmit (TX_CMD).
pub const TX_CMD: u8 = 0x1C;

/// Fixed size of the gen3 transmit-command header, before the 802.11 frame.
pub const TX_CMD_GEN3_HDR: usize = 28;

// Field offsets within the header.
const OFF_LEN: usize = 0;
const OFF_FLAGS: usize = 2;
const OFF_OFFLOAD: usize = 4;
const OFF_RATE_N_FLAGS: usize = 16;

/// Use the rate carried in this command rather than a firmware rate table.
pub const IWL_TX_FLAGS_CMD_RATE: u16 = 1 << 0;
/// The frame is sent as it is: the firmware applies no key to it.
pub const IWL_TX_FLAGS_ENCRYPT_DIS: u16 = 1 << 1;
/// `BIT(TX_CMD_OFFLD_PAD)`: the transport padded the header to four bytes.
pub const TX_CMD_OFFLD_PAD: u32 = 1 << 13;
/// The longest frame the byte count table can describe.
pub const FRAME_MAX: usize = 0x3FFF;

/// Build the transmit command for `frame` (a complete 802.11 frame) at the
/// given rate into `out`, returning the total command length. Returns None if
/// `out` cannot hold the header plus the frame.
#[cfg(test)]
pub fn build(frame: &[u8], rate_n_flags: u32, out: &mut [u8]) -> Option<usize> {
    let total = TX_CMD_GEN3_HDR.checked_add(frame.len())?;
    if out.len() < total {
        return None;
    }
    for b in &mut out[..TX_CMD_GEN3_HDR] {
        *b = 0;
    }
    out[OFF_LEN..OFF_LEN + 2].copy_from_slice(&(frame.len() as u16).to_le_bytes());
    out[OFF_FLAGS..OFF_FLAGS + 2].copy_from_slice(&IWL_TX_FLAGS_CMD_RATE.to_le_bytes());
    out[OFF_RATE_N_FLAGS..OFF_RATE_N_FLAGS + 4].copy_from_slice(&rate_n_flags.to_le_bytes());
    out[TX_CMD_GEN3_HDR..total].copy_from_slice(frame);
    Some(total)
}

/// The header for a `frame_len`-byte frame whose 802.11 header is
/// `hdr_len` bytes, sent at `rate_n_flags` with no firmware key. `None` for
/// a frame longer than [`FRAME_MAX`].
pub fn header(frame_len: usize, hdr_len: usize, rate_n_flags: u32) -> Option<[u8; TX_CMD_GEN3_HDR]> {
    if frame_len > FRAME_MAX {
        return None;
    }
    let mut h = [0u8; TX_CMD_GEN3_HDR];
    h[OFF_LEN..OFF_LEN + 2].copy_from_slice(&(frame_len as u16).to_le_bytes());
    let flags = IWL_TX_FLAGS_CMD_RATE | IWL_TX_FLAGS_ENCRYPT_DIS;
    h[OFF_FLAGS..OFF_FLAGS + 2].copy_from_slice(&flags.to_le_bytes());
    let offload = if hdr_len.is_multiple_of(4) { 0 } else { TX_CMD_OFFLD_PAD };
    h[OFF_OFFLOAD..OFF_OFFLOAD + 4].copy_from_slice(&offload.to_le_bytes());
    h[OFF_RATE_N_FLAGS..OFF_RATE_N_FLAGS + 4].copy_from_slice(&rate_n_flags.to_le_bytes());
    Some(h)
}
