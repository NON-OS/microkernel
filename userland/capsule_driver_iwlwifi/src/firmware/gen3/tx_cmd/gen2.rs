// NONOS Operating System (AGPL-3.0-or-later)

//! The 22000 family's transmit command. Linux v6.12 mvm/tx.c
//! `iwl_mvm_set_tx_params` picks the command by family, not by version:
//! before AX210 it is `iwl_tx_cmd_gen2` (TX_CMD_API_S_VER_7 and _9), 20
//! bytes, `len` 0, `offload_assist` a `__le16` at 2, `flags` a `__le32` at
//! 4, the security info 8..16 and `rate_n_flags` 16. The fields hold what
//! the gen3 command's do (`header`): TX_CMD version 9 is above 8, so the
//! rate is in the version 2 format there too (`iwl_mvm_get_tx_rate`).

use super::{FRAME_MAX, IWL_TX_FLAGS_CMD_RATE, IWL_TX_FLAGS_ENCRYPT_DIS, TX_CMD_OFFLD_PAD};

/// `sizeof(struct iwl_tx_cmd_gen2)`.
pub const TX_CMD_GEN2_HDR: usize = 20;

/// The 22000 family's header for a `frame_len`-byte frame whose 802.11
/// header is `hdr_len` bytes, at `rate_n_flags` with no firmware key.
pub fn header_gen2(
    frame_len: usize,
    hdr_len: usize,
    rate_n_flags: u32,
) -> Option<[u8; TX_CMD_GEN2_HDR]> {
    if frame_len > FRAME_MAX {
        return None;
    }
    let mut h = [0u8; TX_CMD_GEN2_HDR];
    h[0..2].copy_from_slice(&(frame_len as u16).to_le_bytes());
    let offload = if hdr_len.is_multiple_of(4) { 0 } else { TX_CMD_OFFLD_PAD as u16 };
    h[2..4].copy_from_slice(&offload.to_le_bytes());
    let flags = u32::from(IWL_TX_FLAGS_CMD_RATE | IWL_TX_FLAGS_ENCRYPT_DIS);
    h[4..8].copy_from_slice(&flags.to_le_bytes());
    h[16..20].copy_from_slice(&rate_n_flags.to_le_bytes());
    Some(h)
}
